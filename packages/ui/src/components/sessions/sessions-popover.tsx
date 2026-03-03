import { useMemo, useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { useTranslation } from 'react-i18next';
import { Pause, Play, Square, TerminalSquare, Plus } from 'lucide-react';
import { Badge, Button, Popover, PopoverTrigger, PopoverContent, Tooltip, TooltipTrigger, TooltipContent, TooltipProvider } from '@/library';
import { useCurrentProject } from '../../hooks/useProjectQuery';
import { useSessions, useSessionMutations } from '../../hooks/useSessionsQuery';
import type { Session } from '../../types/api';
import { sessionStatusConfig } from '../../lib/session-utils';
import { SessionCreateDialog } from './session-create-dialog';
import { SessionLogsPanel } from './session-logs-panel';

function isActiveSession(session: Session): boolean {
  return session.status === 'running' || session.status === 'pending' || session.status === 'paused';
}

export function SessionsPopover() {
  const { t } = useTranslation('common');
  const navigate = useNavigate();
  const { currentProject } = useCurrentProject();
  const { data: sessions = [] } = useSessions(currentProject?.id ?? null);
  const { startSession, pauseSession, resumeSession, stopSession } = useSessionMutations(currentProject?.id ?? null);

  const [open, setOpen] = useState(false);
  const [createOpen, setCreateOpen] = useState(false);
  const [activeLogSessionId, setActiveLogSessionId] = useState<string | null>(null);

  const activeSessions = useMemo(
    () => sessions.filter(isActiveSession).sort((a, b) => new Date(b.startedAt).getTime() - new Date(a.startedAt).getTime()),
    [sessions]
  );

  const runningCount = useMemo(() => activeSessions.filter((session) => session.status === 'running').length, [activeSessions]);
  const pendingCount = useMemo(() => activeSessions.filter((session) => session.status === 'pending').length, [activeSessions]);
  const hasActive = runningCount > 0 || pendingCount > 0;

  const openHub = () => {
    if (!currentProject?.id) return;
    setOpen(false);
    navigate(`/projects/${currentProject.id}/sessions`);
  };

  const handleCreateOpen = () => {
    setOpen(false);
    setCreateOpen(true);
  };

  return (
    <>
      <Popover open={open} onOpenChange={setOpen}>
        <PopoverTrigger asChild>
          <Button
            variant="ghost"
            size="icon"
            className="relative h-9 w-9 sm:h-10 sm:w-10"
            data-tauri-drag-region="false"
          >
            <TerminalSquare className="h-5 w-5" />
            {hasActive && (
              <span className="absolute top-1.5 right-1.5 flex h-2.5 w-2.5">
                <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
                <span className="relative inline-flex rounded-full h-2.5 w-2.5 bg-green-500"></span>
              </span>
            )}
            <span className="sr-only">{t('sessions.title')}</span>
          </Button>
        </PopoverTrigger>
        <PopoverContent className="w-[800px] max-w-[90vw] p-0" align="end" alignOffset={-8}>
          <div className="flex h-[500px] max-h-[85vh] w-full flex-col md:flex-row">
            <div className="w-full border-b flex flex-col md:w-[380px] shrink-0 md:border-b-0 md:border-r">
              <div className="flex items-center justify-between p-3 border-b">
                <div className="flex items-center gap-2">
                  <span className="text-sm font-semibold">{t('sessions.title')}</span>
                  {(runningCount > 0 || pendingCount > 0) && (
                    <div className="flex items-center gap-2 text-xs">
                      {runningCount > 0 && (
                        <span className="flex items-center gap-1">
                          <span className="h-1.5 w-1.5 rounded-full bg-green-500" />
                          <span className="text-muted-foreground">{runningCount}</span>
                        </span>
                      )}
                      {pendingCount > 0 && (
                        <span className="flex items-center gap-1">
                          <span className="h-1.5 w-1.5 rounded-full bg-amber-500" />
                          <span className="text-muted-foreground">{pendingCount}</span>
                        </span>
                      )}
                    </div>
                  )}
                </div>
                <div className="flex items-center gap-1">
                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <Button size="icon" variant="ghost" className="h-7 w-7" onClick={handleCreateOpen}>
                          <Plus className="h-4 w-4" />
                        </Button>
                      </TooltipTrigger>
                      <TooltipContent>{t('sessions.actions.new')}</TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                  <Button size="sm" variant="link" className="h-7 px-2 text-xs" onClick={openHub}>
                    {t('sessions.actions.viewAll')}
                  </Button>
                </div>
              </div>

              <div className="flex-1 space-y-2 overflow-y-auto p-3">
                {activeSessions.length === 0 ? (
                  <div className="flex flex-col items-center justify-center h-full text-center p-4">
                    <TerminalSquare className="h-8 w-8 text-muted-foreground/50 mb-3" />
                    <p className="text-sm font-medium">{t('sessions.empty')}</p>
                    <p className="text-xs text-muted-foreground mt-1">
                      {t('sessions.labels.emptyHint', 'Start a new session to run specs')}
                    </p>
                    <Button variant="outline" size="sm" className="mt-4" onClick={handleCreateOpen}>
                      {t('sessions.actions.new')}
                    </Button>
                  </div>
                ) : (
                  activeSessions.map((session) => {
                    const statusMeta = sessionStatusConfig[session.status];
                    const StatusIcon = statusMeta.icon;
                    const selected = activeLogSessionId === session.id;

                    return (
                      <button
                        key={session.id}
                        type="button"
                        className={`w-full rounded-md border p-3 mb-2 text-left transition-all ${selected ? 'border-primary bg-primary/5 ring-1 ring-primary/20' : 'hover:border-primary/30 hover:bg-muted/50'}`}
                        onClick={() => setActiveLogSessionId(session.id)}
                      >
                        <div className="mb-1.5 flex items-start justify-between gap-2">
                          <span className="text-sm font-medium leading-none line-clamp-2">{session.prompt?.trim() || session.id.slice(0, 8)}</span>
                          <Badge variant="outline" className={`${statusMeta.className} shrink-0`}>
                            <StatusIcon className="mr-1 h-3 w-3" />
                            {t(`sessions.status.${session.status}`)}
                          </Badge>
                        </div>
                        <div className="mb-3 truncate text-xs text-muted-foreground">
                          {session.specIds.length > 0 ? session.specIds.join(', ') : t('sessions.select.empty')}
                        </div>
                        <div className="flex items-center gap-1.5 mt-auto">
                          {session.status === 'pending' && (
                            <Button size="sm" variant="secondary" className="h-7 px-2.5 text-xs w-full" onClick={(event) => {
                              event.stopPropagation();
                              void startSession(session.id);
                            }}>
                              <Play className="mr-1.5 h-3.5 w-3.5" />
                              {t('sessions.actions.start')}
                            </Button>
                          )}
                          {session.status === 'running' && (
                            <>
                              <Button size="sm" variant="secondary" className="h-7 px-2.5 text-xs flex-1" onClick={(event) => {
                                event.stopPropagation();
                                void pauseSession(session.id);
                              }}>
                                <Pause className="mr-1.5 h-3.5 w-3.5" />
                                {t('sessions.actions.pause')}
                              </Button>
                              <Button size="sm" variant="destructive" className="h-7 px-2.5 text-xs flex-1" onClick={(event) => {
                                event.stopPropagation();
                                void stopSession(session.id);
                              }}>
                                <Square className="mr-1.5 h-3 w-3" />
                                {t('sessions.actions.stop')}
                              </Button>
                            </>
                          )}
                          {session.status === 'paused' && (
                            <>
                              <Button size="sm" variant="secondary" className="h-7 px-2.5 text-xs flex-1" onClick={(event) => {
                                event.stopPropagation();
                                void resumeSession(session.id);
                              }}>
                                <Play className="mr-1.5 h-3.5 w-3.5" />
                                {t('sessions.actions.resume')}
                              </Button>
                              <Button size="sm" variant="destructive" className="h-7 px-2.5 text-xs flex-1" onClick={(event) => {
                                event.stopPropagation();
                                void stopSession(session.id);
                              }}>
                                <Square className="mr-1.5 h-3 w-3" />
                                {t('sessions.actions.stop')}
                              </Button>
                            </>
                          )}
                        </div>
                      </button>
                    );
                  })
                )}
              </div>
            </div>

            <div className="flex-1 p-0 flex flex-col min-h-0 bg-muted/10 relative overflow-hidden">
              {activeLogSessionId ? (
                <SessionLogsPanel
                  sessionId={activeLogSessionId}
                  onBack={() => setActiveLogSessionId(null)}
                />
              ) : (
                <div className="flex flex-col h-full items-center justify-center p-6 text-center text-muted-foreground w-full">
                  <TerminalSquare className="h-10 w-10 mb-4 opacity-20" />
                  <p className="text-sm">{t('sessions.labels.logs')}</p>
                  <p className="text-xs mt-1 max-w-[250px] opacity-70">
                    {t('sessions.labels.logsHint', 'Select an active session from the left to view its execution logs')}
                  </p>
                </div>
              )}
            </div>
          </div>
        </PopoverContent>
      </Popover>

      <SessionCreateDialog
        open={createOpen}
        onOpenChange={setCreateOpen}
        projectPath={currentProject?.path}
      />
    </>
  );
}
