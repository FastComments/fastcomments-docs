[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Deze route keert elke migratie van één [`POST /api/v1/pages/migrations`](#pages-migrations-post) taak terug. Haal de `undoId` op van de voltooide migratietaak met [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Een `undoId` kan 90 dagen worden gebruikt, en slechts één keer.

Het ongedaan maken wordt uitgevoerd als een batchtaak, net als de migratie, en het antwoord bevat de `batchJobId`. Zodra de taak is voltooid, rapporteert zijn `pageMigrationUndoResult`:

- `restored`: records die teruggezet zijn naar hun waarden van vóór de migratie.
- `skipped`: records die ongemoeid zijn gelaten omdat ze na de migratie zijn gewijzigd, of niet meer bestaan. Een latere bewerking wordt nooit overschreven.
- `notRestorable`: records die door de migratie zijn verwijderd tijdens het samenvoegen van twee pagina's, of het verwijderen van dubbele paginareacties. Deze kunnen niet worden teruggehaald.

Migraties worden omgekeerd, nieuwste eerst, dus een keten zoals `a` naar `b` gevolgd door `b` naar `c` wordt teruggedraaid naar `a`. Een migratietaak die halverwege is mislukt, kan nog steeds ongedaan worden gemaakt, waardoor de wijzigingen die vóór het falen zijn aangebracht, worden teruggedraaid.

Het in de wachtrij plaatsen van een ongedaan maken kost niets. Wanneer de taak voltooid is, kost het `1` credit per `1000` herstelde records, met een minimum van `1`.

[inline-code-attrs-start title = 'Voorbeeld cURL voor het ongedaan maken van paginamigraties'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van het verzoek voor het ongedaan maken van paginamigraties'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de respons voor het ongedaan maken van paginamigraties'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Included on failure.
     * undo-not-found (404): no migration has this undoId, or it is older than 90 days.
     * already-undone (409) and undo-in-progress (409): the migration has been undone, or an undo of it is queued or running.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Included on failure. **/
    reason?: string
    /** Pass to GET /api/v1/batch-jobs/:id for progress and the result. **/
    batchJobId?: string
}
[inline-code-end]