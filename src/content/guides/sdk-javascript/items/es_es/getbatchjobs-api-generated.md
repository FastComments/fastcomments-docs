Los trabajos por lotes del tenant (importaciones, exportaciones, migraciones de páginas...), los más recientes primero.

## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| type | string | No |  |
| status | string | No |  |
| skip | number | No |  |

## Respuesta

Devuelve: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Obtener todos los trabajos por lotes
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Obtener los trabajos de migración de páginas fallidos, saltando los primeros 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]