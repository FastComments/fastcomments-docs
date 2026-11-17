Pone en cola un trabajo por lotes que revierte cada migración de un trabajo migrate-pages, del más reciente al más antiguo. Disponible durante 90 días.
Los registros modificados desde la migración se dejan intactos, por lo que una edición posterior nunca se sobrescribe.
Consulte GET /api/v1/batch-jobs/{batchJobId} para obtener los recuentos de restaurados/omitidos.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| undoId | string | Sí |  |

## Respuesta

Devuelve: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/QueuePageMigrationJobAPIResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de undoPageMigrations'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = '5a3a58e84f5a2c1d34e8b9a7';

async function run(): Promise<void> {
  const undoId: string = 'migration-undo-2024-06-12-001';
  const result: QueuePageMigrationJobAPIResponse = await undoPageMigrations(tenantId, undoId);
  console.log(result.status === 'success' ? 'Undo queued' : 'Undo failed');
}

run();
[inline-code-end]