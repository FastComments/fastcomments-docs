Paketni opravila najemnika (uvozi, izvozi, migracije strani...), od najnovejšega proti najstarejšemu.

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| type | string | Ne |  |
| status | string | Ne |  |
| skip | number | Ne |  |

## Odgovor

Vrne: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Pridobi vsa paketna opravila
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Pridobi neuspela opravila migracije strani, s preskočitvijo prvih 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]