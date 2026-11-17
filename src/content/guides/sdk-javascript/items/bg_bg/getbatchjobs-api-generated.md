Пакетните задачи на наемателя (импорти, експорти, миграции на страници...), най-новите първо.

## Параметри

| Име | Тип | Задължителен | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| type | string | Не |  |
| status | string | Не |  |
| skip | number | Не |  |

## Отговор

Връща: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Получаване на всички пакетни задачи
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Получаване на неуспешни задачи за миграция на страници, пропускайки първите 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]