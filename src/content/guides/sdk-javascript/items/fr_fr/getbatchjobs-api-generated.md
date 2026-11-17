Les tâches par lots du tenant (imports, exports, migrations de pages...), de la plus récente à la plus ancienne.

## Paramètres

| Nom | Type | Requis | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| type | string | Non |  |
| status | string | Non |  |
| skip | number | Non |  |

## Réponse

Retourne : [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Récupérer toutes les tâches par lots
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Récupérer les tâches de migration de pages en échec, en ignorant les 10 premières
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]