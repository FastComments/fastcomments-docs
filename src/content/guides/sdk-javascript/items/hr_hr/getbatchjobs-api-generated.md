Grupni poslovi klijenta (uvozi, izvozi, migracije stranica...), od najnovijih.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| type | string | Ne |  |
| status | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vraća: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getBatchJobs primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Dohvati sve grupne poslove
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Dohvati neuspjele poslove migracije stranica, preskačući prvih 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]