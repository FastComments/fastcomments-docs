I processi batch del tenant (importazioni, esportazioni, migrazioni di pagine...), dal più recente.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| type | string | No |  |
| status | string | No |  |
| skip | number | No |  |

## Risposta

Restituisce: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Ottieni tutti i processi batch
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Ottieni i processi di migrazione pagine falliti, saltando i primi 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]