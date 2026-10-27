[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Esta rota reverte todas as migrações de um job [`POST /api/v1/pages/migrations`](#pages-migrations-post). Obtenha o `undoId` do job de migração concluído com [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Um `undoId` pode ser usado por 90 dias e apenas uma vez.

O desfazer é executado como um job em lote, assim como a migração, e a resposta contém seu `batchJobId`. Quando o job termina, seu `pageMigrationUndoResult` relata:

- `restored`: registros devolvidos aos seus valores anteriores à migração.
- `skipped`: registros deixados como estão porque foram alterados após a migração, ou não existem mais. Uma edição posterior nunca é sobrescrita.
- `notRestorable`: registros que a migração excluiu ao mesclar duas páginas ou ao remover reações de página duplicadas. Esses não podem ser recuperados.

As migrações são revertidas da mais recente para a mais antiga, portanto uma cadeia como `a` para `b` seguida de `b` para `c` é desfeita de volta para `a`. Um job de migração que falhou parcialmente ainda pode ser desfeito, o que reverte as alterações feitas antes da falha.

Enfileirar um desfazer não custa nada. Quando o job termina, custa `1` crédito por `1000` registros restaurados, com um mínimo de `1`.

[inline-code-attrs-start title = 'Exemplo cURL de Desfazer Migrações de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Requisição de Desfazer Migrações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estrutura da Resposta de Desfazer Migrações de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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