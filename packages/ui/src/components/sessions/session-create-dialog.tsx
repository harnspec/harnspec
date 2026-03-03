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
import type { Session, SessionMode, Spec } from '../../types/api';
import { api } from '../../lib/api';
import { SpecContextTrigger, SpecContextChips } from '../spec-context-attachments';
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
  const [runners, setRunners] = useState<string[]>([]);
  const [runner, setRunner] = useState('claude');
  const [mode, setMode] = useState<SessionMode>('autonomous');
  const [selectedSpecIds, setSelectedSpecIds] = useState<string[]>(defaultSpecId ? [defaultSpecId] : []);
  const [promptTemplate, setPromptTemplate] = useState('');
  const [specs, setSpecs] = useState<Spec[]>([]);
  const [creating, setCreating] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const inputRef = useRef<HTMLTextAreaElement>(null);

  const canCreate = Boolean(projectPath);

  useEffect(() => {
    setSelectedSpecIds(defaultSpecId ? [defaultSpecId] : []);
  }, [defaultSpecId]);

  useEffect(() => {
    if (!open) return;
    setError(null);
    const loadRunners = async () => {
      try {
        const available = await api.listAvailableRunners(projectPath ?? undefined);
        setRunners(available.length ? available : ['claude', 'copilot', 'codex', 'opencode', 'aider', 'cline']);
      } catch {
        setRunners(['claude', 'copilot', 'codex', 'opencode', 'aider', 'cline']);
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
  }, [projectPath, selectedSpecIds, promptTemplate, runner, mode, onCreated, onOpenChange, t]);

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
                    <PromptInputSelectValue placeholder={t('sessions.labels.runner')} />
                  </PromptInputSelectTrigger>
                  <PromptInputSelectContent>
                    {runners.map((runnerValue) => (
                      <PromptInputSelectItem key={runnerValue} value={runnerValue}>
                        {runnerValue}
                      </PromptInputSelectItem>
                    ))}
                  </PromptInputSelectContent>
                </PromptInputSelect>

                <PromptInputSelect value={mode} onValueChange={(value) => setMode(value as SessionMode)}>
                  <PromptInputSelectTrigger className="h-8 w-auto rounded-full border border-border/70 px-3 py-1.5 text-xs">
                    <PromptInputSelectValue placeholder={t('sessions.labels.mode')} />
                  </PromptInputSelectTrigger>
                  <PromptInputSelectContent>
                    {MODES.map((modeValue) => (
                      <PromptInputSelectItem key={modeValue} value={modeValue}>
                        {t(`sessions.modes.${modeValue}`)}
                      </PromptInputSelectItem>
                    ))}
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
