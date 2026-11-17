Пакетні завдання тенанта (імпорти, експорти, міграції сторінок...), від найновіших до найстаріших.

## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| type | string | Ні |  |
| status | string | Ні |  |
| skip | number | Ні |  |

## Відповідь

Повертає: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Отримати всі пакетні завдання
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Отримати невдалі завдання міграції сторінок, пропустивши перші 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]