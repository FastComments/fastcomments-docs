Lejemandens batchjobs (imports, eksporter, sidemigreringer...), nyeste først.

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| type | string | Nej |  |
| status | string | Nej |  |
| skip | number | Nej |  |

## Svar

Returnerer: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getBatchJobs Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Hent alle batchjobs
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Hent fejlende sidemigreringsjobs, og spring de første 10 over
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]