[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Denne rute vender hver migration fra et [`POST /api/v1/pages/migrations`](#pages-migrations-post) job om. Hent `undoId` fra det afsluttede migrationsjob med [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Et `undoId` kan bruges i 90 dage, og kun én gang.

Undoen kører som et batch-job, ligesom migrationen, og svaret indeholder dens `batchJobId`. Når jobbet er færdigt, rapporterer dens `pageMigrationUndoResult`:

- `restored`: poster sat tilbage til deres værdier fra før migrationen.
- `skipped`: poster efterladt uændret fordi de ændrede sig efter migrationen, eller ikke længere findes. En senere redigering overskrives aldrig.
- `notRestorable`: poster som migrationen slettede mens to sider blev flettet, eller duplikerede side‑reaktioner fjernet. Disse kan ikke bringes tilbage.

Migrationer vendes om i rækkefølge fra nyeste til ældste, så en kæde som `a` til `b` efterfulgt af `b` til `c` rulles tilbage til `a`. Et migrationsjob, der fejlede delvist, kan stadig fortrydes, hvilket vender de ændringer, det foretog før fejlen, om.

At sætte en fortrydelse i kø koster intet. Når jobbet er færdigt, koster det `1` kredit per `1000` poster gendannet, med et minimum på `1`.

[inline-code-attrs-start title = 'Fortryd side-migrationer cURL-eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Fortryd side-migrationer anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Fortryd side-migrationer svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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