Пакетные задания тенанта (импорты, экспорты, миграции страниц...), сначала самые новые.

## Параметры

| Имя | Тип | Обязательный | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| type | string | Нет |  |
| status | string | Нет |  |
| skip | number | Нет |  |

## Ответ

Возвращает: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Получить все пакетные задания
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Получить неудачные задания миграции страниц, пропустив первые 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]