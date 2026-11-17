Batch послови тенанта (увози, извози, миграције страница...), од најновијих.

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| type | string | Не |  |
| status | string | Не |  |
| skip | number | Не |  |

## Одговор

Враћа: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'getBatchJobs пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Преузми све batch послове
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Преузми неуспеле послове миграције страница, прескачући првих 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]