[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Diese Route kehrt jede Migration von einem [`POST /api/v1/pages/migrations`](#pages-migrations-post) Job um. Holen Sie die `undoId` aus dem abgeschlossenen Migrations‑Job mit [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Eine `undoId` kann 90 Tage lang verwendet werden und nur einmal.

Der Undo wird als Batch‑Job ausgeführt, genau wie die Migration, und die Antwort enthält dessen `batchJobId`. Sobald der Job abgeschlossen ist, berichtet sein `pageMigrationUndoResult`:

- `restored`: Datensätze, die auf ihre Werte vor der Migration zurückgesetzt wurden.
- `skipped`: Datensätze, die unverändert bleiben, weil sie nach der Migration geändert wurden oder nicht mehr existieren. Eine spätere Bearbeitung wird nie überschrieben.
- `notRestorable`: Datensätze, die die Migration beim Zusammenführen zweier Seiten oder beim Entfernen doppelter Seitenreaktionen gelöscht hat. Diese können nicht wiederhergestellt werden.

Migrationen werden in umgekehrter Reihenfolge (neueste zuerst) rückgängig gemacht, sodass eine Kette wie `a` zu `b` gefolgt von `b` zu `c` wieder zurück zu `a` entwirrt wird. Ein Migrations‑Job, der teilweise fehlgeschlagen ist, kann dennoch rückgängig gemacht werden, wodurch die vor dem Fehlschlag vorgenommenen Änderungen rückgängig werden.

Das Einreihen eines Undos kostet nichts. Wenn der Job abgeschlossen ist, kostet es `1` Credit pro `1000` wiederhergestellte Datensätze, mindestens `1`.

[inline-code-attrs-start title = 'Beispiel für cURL zum Rückgängigmachen von Seitenmigrationen'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Anforderung zum Rückgängigmachen von Seitenmigrationen'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur der Antwort zum Rückgängigmachen von Seitenmigrationen'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Bei einem Fehler enthalten.
     * undo-not-found (404): Keine Migration hat diese undoId, oder sie ist älter als 90 Tage.
     * already-undone (409) und undo-in-progress (409): Die Migration wurde bereits rückgängig gemacht, oder ein Undo dafür ist in der Warteschlange oder läuft.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Bei einem Fehler enthalten. **/
    reason?: string
    /** An GET /api/v1/batch-jobs/:id übergeben für Fortschritt und Ergebnis. **/
    batchJobId?: string
}
[inline-code-end]