Zadania wsadowe dzierżawy (importy, eksporty, migracje stron...), od najnowszych.

## Parametry

| Nazwa | Typ | Wymagany | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| type | string | Nie |  |
| status | string | Nie |  |
| skip | number | Nie |  |

## Odpowiedź

Zwraca: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Pobierz wszystkie zadania wsadowe
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Pobierz nieudane zadania migracji stron, pomijając pierwsze 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]