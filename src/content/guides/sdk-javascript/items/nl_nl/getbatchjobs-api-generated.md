De batchtaken van de tenant (imports, exports, paginamigraties...), nieuwste eerst.

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| type | string | Nee |  |
| status | string | Nee |  |
| skip | number | Nee |  |

## Response

Geeft terug: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getBatchJobs Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Alle batchtaken ophalen
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Mislukte paginamigratietaken ophalen, waarbij de eerste 10 worden overgeslagen
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]