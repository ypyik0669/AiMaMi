import { useState } from "react";
import { useQuery } from "@tanstack/react-query";
import { useTranslation } from "react-i18next";
import { ArrowRight, CheckCircle2, FileCode2, FolderOpen, RefreshCw, Server, Settings, Sparkles, TriangleAlert, Wrench } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Skeleton } from "@/components/ui/skeleton";
import { api } from "@/lib/api";
import type { Route } from "@/types/navigation";

export function OverviewPage({ onNavigate }: { onNavigate: (route: Route) => void }) {
  const { t, i18n } = useTranslation();
  const [openError, setOpenError] = useState<string | null>(null);
  const snapshot = useQuery({
    queryKey: ["runtime-state", "display"],
    queryFn: () => api.loadSnapshot(),
  });
  const status = snapshot.data?.data.status;
  const links = [
    { route: "mcp", icon: Server },
    { route: "skills", icon: Sparkles },
    { route: "customInstructions", icon: FileCode2 },
    { route: "maintenance", icon: Wrench },
    { route: "settings", icon: Settings },
  ] as const;

  const openHome = async () => {
    if (!status) return;
    setOpenError(null);
    try {
      await api.openPath(status.paths.codexHome);
    } catch (error) {
      setOpenError(String(error));
    }
  };

  return (
    <div className="mx-auto max-w-5xl space-y-8 pb-4">
      <header className="flex flex-wrap items-center justify-between gap-3 border-b pb-4">
        <div>
          <h1 className="text-xl font-semibold">{t("overview.title")}</h1>
          {status && (
            <p className="mt-1 text-xs text-muted-foreground">
              {t("overview.lastUpdated", {
                time: new Date(status.lastScanAt * 1000).toLocaleTimeString(i18n.language),
              })}
            </p>
          )}
        </div>
        <Button variant="outline" size="icon" onClick={() => void snapshot.refetch()}
          disabled={snapshot.isFetching} title={t("common.refresh")} aria-label={t("common.refresh")}>
          <RefreshCw className={`size-4 ${snapshot.isFetching ? "animate-spin motion-reduce:animate-none" : ""}`} />
        </Button>
      </header>

      {snapshot.isPending && (
        <div role="status" aria-label={t("common.loading")} className="space-y-4">
          <Skeleton className="h-5 w-40" />
          <Skeleton className="h-16 w-full" />
          <Skeleton className="h-16 w-full" />
        </div>
      )}
      {snapshot.isError && (
        <div role="alert" className="space-y-3 border-l-2 border-destructive pl-4">
          <p className="font-medium">{t("overview.loadFailed")}</p>
          <p className="break-all text-sm text-muted-foreground">{String(snapshot.error)}</p>
          <Button variant="outline" onClick={() => void snapshot.refetch()} disabled={snapshot.isFetching}>
            <RefreshCw className="mr-2 size-4" />{t("common.retry")}
          </Button>
        </div>
      )}
      {status && (
        <section className="space-y-4" aria-label={t("overview.healthTitle")}>
          <div className="flex flex-wrap items-center justify-between gap-2">
            <h2 className="text-sm font-semibold">{t("overview.healthTitle")}</h2>
            <Button variant="ghost" size="sm" onClick={() => void openHome()}>
              <FolderOpen className="mr-2 size-4" />{t("overview.openCodexFolder")}
            </Button>
          </div>
          <p className="break-all font-mono text-xs text-muted-foreground">{status.paths.codexHome}</p>
          <dl className="divide-y border-y">
            {[
              { label: t("overview.healthAuth"), path: status.paths.authPath, exists: status.paths.authExists },
              { label: t("overview.healthRegistry"), path: status.paths.registryPath, exists: status.paths.registryExists },
              { label: t("overview.healthSessions"), path: status.paths.sessionsPath, exists: status.paths.sessionsExists },
            ].map((item) => (
              <div key={item.path} className="grid grid-cols-[minmax(0,1fr)_auto] items-center gap-4 py-4">
                <dt className="min-w-0">
                  <span className="text-sm font-medium">{item.label}</span>
                  <span className="mt-1 block break-all font-mono text-xs text-muted-foreground">{item.path}</span>
                </dt>
                <dd className={`flex items-center gap-2 text-xs ${item.exists ? "text-emerald-600 dark:text-emerald-400" : "text-muted-foreground"}`}>
                  {item.exists ? <CheckCircle2 className="size-4" /> : <TriangleAlert className="size-4" />}
                  {t(item.exists ? "overview.healthOk" : "overview.healthMissing")}
                </dd>
              </div>
            ))}
          </dl>
          {openError && <p role="alert" className="break-all text-sm text-destructive">{openError}</p>}
        </section>
      )}

      <nav aria-label={t("overview.quickAccess")} className="space-y-3">
        <h2 className="text-sm font-semibold">{t("overview.quickAccess")}</h2>
        <div className="grid gap-x-6 sm:grid-cols-2">
          {links.map(({ route, icon: Icon }) => (
            <button key={route} type="button" onClick={() => onNavigate(route)}
              className="flex min-h-14 items-center gap-3 border-b px-1 py-3 text-left text-sm hover:bg-muted/50 focus-visible:outline focus-visible:outline-2 focus-visible:outline-ring">
              <Icon className="size-4 shrink-0 text-muted-foreground" />
              <span className="min-w-0 flex-1 break-words">{t(`nav.${route}`)}</span>
              <ArrowRight className="size-4 shrink-0 text-muted-foreground" />
            </button>
          ))}
        </div>
      </nav>
    </div>
  );
}
