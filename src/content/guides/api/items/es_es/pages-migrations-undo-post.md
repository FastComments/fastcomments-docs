[api-resource-header-start name = 'Page Migrations'; route = 'POST /api/v1/pages/migrations/:undoId/undo'; creditsCost = 1; api-resource-header-end]

Esta ruta revierte cada migración de un trabajo [`POST /api/v1/pages/migrations`](#pages-migrations-post). Obtén el `undoId` del trabajo de migración finalizado con [`GET /api/v1/batch-jobs/:id`](#batch-job-get-by-id). Un `undoId` puede usarse durante 90 días, y solo una vez.

El deshacer se ejecuta como un trabajo por lotes, igual que la migración, y la respuesta contiene su `batchJobId`. Una vez que el trabajo termina, su `pageMigrationUndoResult` informa:

- `restored`: registros devueltos a sus valores anteriores a la migración.
- `skipped`: registros dejados sin cambios porque se modificaron después de la migración, o ya no existen. Una edición posterior nunca se sobrescribe.
- `notRestorable`: registros que la migración eliminó al combinar dos páginas, o al eliminar reacciones de página duplicadas. No pueden recuperarse.

Las migraciones se revierten de la más reciente a la más antigua, por lo que una cadena como `a` a `b` seguida de `b` a `c` se deshace volviendo a `a`. Un trabajo de migración que falló a mitad puede aún deshacerse, lo que revierte los cambios realizados antes del fallo.

Encolar un deshacer no cuesta nada. Cuando el trabajo finaliza, cuesta `1` crédito por cada `1000` registros restaurados, con un mínimo de `1`.

[inline-code-attrs-start title = 'Ejemplo cURL para Deshacer Migraciones de Página'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/pages/migrations/my-undo-id/undo?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Solicitud para Deshacer Migraciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Estructura de la Respuesta para Deshacer Migraciones de Página'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface UndoPageMigrationsResponse {
    status: 'success' | 'failed'
    /**
     * Incluido en caso de error.
     * undo-not-found (404): ninguna migración tiene este undoId, o es más antiguo que 90 días.
     * already-undone (409) y undo-in-progress (409): la migración ha sido deshecha, o una reversión está en cola o en ejecución.
     **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'undo-not-found' | 'already-undone' | 'undo-in-progress' | 'internal'
    /** Incluido en caso de error. **/
    reason?: string
    /** Pase a GET /api/v1/batch-jobs/:id para progreso y el resultado. **/
    batchJobId?: string
}
[inline-code-end]

---