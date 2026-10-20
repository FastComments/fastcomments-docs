[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Cette route annule chaque migration d’un travail [`POST /api/v1/pages/migrations`](#pages-migrations-post). Récupérez le `undoId` du travail de migration terminé avec [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Un `undoId` peut être utilisé pendant 90 jours, et uniquement une fois.

L’annulation s’exécute comme un travail batch, de la même manière que la migration, et la réponse contient son `batchJobId`. Une fois le travail terminé, son `pageMigrationUndoResult` indique :

- `restored` : enregistrements remis à leurs valeurs d’avant la migration.
- `skipped` : enregistrements laissés tels quels parce qu’ils ont changé après la migration, ou n’existent plus. Une modification ultérieure n’est jamais écrasée.
- `notRestorable` : enregistrements que la migration a supprimés lors de la fusion de deux pages, ou en supprimant les réactions de page en double. Ceux‑ci ne peuvent pas être restaurés.

Les migrations sont annulées de la plus récente à la plus ancienne, ainsi une chaîne comme `a` vers `b` suivie de `b` vers `c` est déroulée jusqu’à `a`. Un travail de migration qui a échoué partiellement peut encore être annulé, ce qui inverse les modifications qu’il a effectuées avant l’échec.

Mettre en file d’attente une annulation ne coûte rien. Lorsque le travail se termine, il coûte `1` crédit par `1000` enregistrements restaurés, avec un minimum de `1`.

[inline-code-attrs-start title = 'Exemple cURL d\'annulation des migrations de page'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la requête d\'annulation des migrations de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structure de la réponse d\'annulation des migrations de page'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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