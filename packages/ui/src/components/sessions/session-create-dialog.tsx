import { useCallback, useEffect, useRef, useState } from 'react';
import {
  Alert,
  AlertDescription,
  Button,
  PromptInput,
  PromptInputBody,
  PromptInputFooter,
  PromptInputSelect,
  PromptInputSelectContent,
  PromptInputSelectItem,
  PromptInputSelectTrigger,
  PromptInputSelectValue,
  PromptInputSubmit,
  PromptInputTextarea,
} from '@/library';
import { useTranslation } from 'react-i18next';
import type { Session, SessionMode, Spec, RunnerDefinition } from '../../types/api';
import { api } from '../../lib/api';
import { SpecContextTrigger, SpecContextChips } from '../spec-context-attachments';
import { RunnerLogo } from '../library/ai-elements/runner-logo';
import { InlineModelSelector } from '../chat/inline-model-selector';
import { sessionModeConfig } from '../../lib/session-utils';
import { X } from 'lucide-react';

const MODES: SessionMode[] = ['guided', 'autonomous']; // 'ralph' is deprecated

interface SessionCreateDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  projectPath?: string | null;
  defaultSpecId?: string | null;
  onCreated?: (session: Session) => void;
}

export function SessionCreateDialog({
  open,
  onOpenChange,
  projectPath,
  defaultSpecId,
  onCreated,
}: SessionCreateDialogProps) {
  const { t } = useTranslation('common');
  const [runnerDefs, setRunnerDefs] = useState<RunnerDefinition[]>([]);
  const [runner, setRunner] = useState('');
  const [modelSelection, setModelSelection] = useState<{ providerId: string; modelId: string } | undefined>();
  const [mode, setMode] = useState<SessionMode>('autonomous');
  const [selectedSpecIds, setSelectedSpecIds] = useState<string[]>(defaultSpecId ? [defaultSpecId] : []);
  const [promptTemplate, setPromptTemplate] = useState('');
  const [specs, setSpecs] = useState<Spec[]>([]);
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  const canCreate = Boolean(projectPath);

  /** Get display label for a runner */
  const getRunnerLabel = useCallback(
    (runnerId: string) => {
      const key = `sessions.runners.${runnerId}` as const;
      const translated = t(key);
      // If i18n returns the key itself, fall back to the runner definition name or the id
      if (translated === key || translated === `sessions.runners.${runnerId}`) {
        const def = runnerDefs.find((r) => r.id === runnerId);
        return def?.name ?? runnerId;
      }
      return translated;
    },
    [t, runnerDefs],
  );

  useEffect(() => {
    setSelectedSpecIds(defaultSpecId ? [defaultSpecId] : []);
  }, [defaultSpecId]);

  useEffect(() => {
    if (!open) return;
    setError(null);
    const loadRunners = async () => {
      try {
        const resp = await api.listRunners(projectPath ?? undefined, { skipValidation: true });
        const defs = resp.runners.length
          ? resp.runners
          : (['claude', 'copilot', 'codex', 'opencode', 'aider', 'cline'] as const).map(
            (id) => ({ id, args: [], env: {}, source: 'builtin' as const }),
          );
        setRunnerDefs(defs);
        // Set the default runner: prefer server-configured default, else first available
        const defaultId = resp.default ?? defs[0]?.id ?? 'claude';
        setRunner((prev) => (prev && defs.some((d) => d.id === prev) ? prev : defaultId));
      } catch {
        const fallback: RunnerDefinition[] = (['claude', 'copilot', 'codex', 'opencode', 'aider', 'cline'] as const).map(
          (id) => ({ id, args: [], env: {}, source: 'builtin' as const }),
        );
        setRunnerDefs(fallback);
        setRunner((prev) => prev || 'claude');
      }
    };
    const loadSpecs = async () => {
      try {
        const data = await api.getSpecs();
        setSpecs(data);
      } catch {
        // Best-effort; spec picker will be empty
      }
    };
    void loadRunners();
    void loadSpecs();
  }, [open, projectPath]);

  useEffect(() => {
    if (!open) {
      return;
    }
    setError(null);
    setTimeout(() => inputRef.current?.focus(), 50);
  }, [open]);

  const runCreate = useCallback(async () => {
    if (!projectPath) return;
    setCreating(true);
    setError(null);
    try {
      const created = await api.createSession({
        projectPath,
        specIds: selectedSpecIds,
        prompt: promptTemplate.trim() || null,
        runner,
        mode,
        model: modelSelection?.modelId || undefined,
      });
      // Start the runtime in the background — the server returns immediately
      // and the session transitions from Pending to Running asynchronously.
      void api.startSession(created.id);
      onCreated?.(created);
      onOpenChange(false);
    } catch (err) {
      setError(err instanceof Error ? err.message : t('sessions.errors.create'));
      throw err;
    } finally {
      setCreating(false);
    }
  }, [projectPath, selectedSpecIds, promptTemplate, runner, mode, modelSelection, onCreated, onOpenChange, t]);

  if (!open) {
    return null;
  }

  return (
    <div
      className="fixed inset-0 z-50 flex items-start justify-center bg-background/60 px-4 pt-20 backdrop-blur-sm"
      onClick={(e) => {
        if (e.target === e.currentTarget) {
          onOpenChange(false);
        }
      }}
    >
      <div className="w-[min(860px,96vw)] rounded-xl border bg-background shadow-2xl">
        <div className="flex items-center justify-between border-b px-4 py-3">
          <div>
            <h2 className="text-sm font-semibold">{t('sessions.dialogs.createTitle')}</h2>
            <p className="text-xs text-muted-foreground">{t('sessions.dialogs.createDescription')}</p>
          </div>
          <Button variant="ghost" size="icon" className="h-8 w-8" onClick={() => onOpenChange(false)}>
            <X className="h-4 w-4" />
          </Button>
        </div>

        <div className="space-y-3 p-4">
          {error && (
            <Alert variant="destructive">
              <AlertDescription>{error}</AlertDescription>
            </Alert>
          )}

          <div>
            <SpecContextChips
              specs={specs}
              selectedSpecIds={selectedSpecIds}
              onSelectedSpecIdsChange={setSelectedSpecIds}
              className="pb-2"
            />
          </div>

          <PromptInput onSubmit={() => void runCreate()}>
            <PromptInputBody>
              <PromptInputTextarea
                ref={inputRef}
                value={promptTemplate}
                onChange={(e) => setPromptTemplate(e.target.value)}
                placeholder={t('sessions.labels.promptPlaceholder')}
                disabled={creating}
                className="min-h-28"
              />
            </PromptInputBody>

            <PromptInputFooter>
              <div className="flex flex-wrap items-center gap-2">
                <SpecContextTrigger
                  specs={specs}
                  selectedSpecIds={selectedSpecIds}
                  onSelectedSpecIdsChange={setSelectedSpecIds}
                  searchPlaceholder={t('sessions.select.search')}
                  emptyLabel={t('sessions.select.empty')}
                  triggerLabel={t('sessions.labels.attachSpec')}
                />

                <PromptInputSelect value={runner} onValueChange={setRunner}>
                  <PromptInputSelectTrigger className="h-8 w-auto rounded-full border border-border/70 px-3 py-1.5 text-xs">
                    <span className="flex items-center gap-1.5">
                      <RunnerLogo runnerId={runner} size={16} className="rounded-sm" />
                      <PromptInputSelectValue placeholder={t('sessions.labels.runner')} />
                    </span>
                  </PromptInputSelectTrigger>
                  <PromptInputSelectContent>
                    {runnerDefs.map((def) => (
                      <PromptInputSelectItem key={def.id} value={def.id}>
                        <span className="flex items-center gap-2">
                          <RunnerLogo runnerId={def.id} size={16} className="rounded-sm" />
                          {getRunnerLabel(def.id)}
                        </span>
                      </PromptInputSelectItem>
                    ))}
                  </PromptInputSelectContent>
                </PromptInputSelect>

                <InlineModelSelector
                  value={modelSelection}
                  onChange={setModelSelection}
                  disabled={creating}
                />

                <PromptInputSelect value={mode} onValueChange={(value) => setMode(value as SessionMode)}>
                  <PromptInputSelectTrigger className="h-8 w-auto rounded-full border border-border/70 px-3 py-1.5 text-xs">
                    <span className="flex items-center gap-1.5">
                      {(() => {
                        const ModeIcon = sessionModeConfig[mode]?.icon;
                        return ModeIcon ? <ModeIcon className="h-3.5 w-3.5" /> : null;
                      })()}
                      <PromptInputSelectValue placeholder={t('sessions.labels.mode')} />
                    </span>
                  </PromptInputSelectTrigger>
                  <PromptInputSelectContent>
                    {MODES.map((modeValue) => {
                      const ModeIcon = sessionModeConfig[modeValue]?.icon;
                      return (
                        <PromptInputSelectItem key={modeValue} value={modeValue}>
                          <span className="flex items-center gap-2">
                            {ModeIcon && <ModeIcon className="h-3.5 w-3.5" />}
                            {t(`sessions.modes.${modeValue}`)}
                          </span>
                        </PromptInputSelectItem>
                      );
                    })}
                  </PromptInputSelectContent>
                </PromptInputSelect>
              </div>

              <PromptInputSubmit
                disabled={!canCreate || creating || (!promptTemplate.trim() && selectedSpecIds.length === 0)}
                status={creating ? 'submitted' : undefined}
              />
            </PromptInputFooter>
          </PromptInput>

        </div>
      </div>
    </div>
  );
}
