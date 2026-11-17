Статус, напредак и, када се заврши, резултат једног batch посла.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Одговор

Враћа: [`GetBatchJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobAPIResponse.ts)

## Пример

[inline-code-attrs-start title = 'getBatchJob пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const batchJobId: string = 'job_5f8a9b2c3d4e5f6a7b8c9d0e';

const response: GetBatchJobAPIResponse = await getBatchJob(tenantId, batchJobId);

const job: APIBatchJob = response.job;
const undoCounts: PageMigrationUndoCounts | undefined = job.pageMigrationResults?.[0]?.undoCounts;

console.log(`Batch job ${job.id} status: ${job.status}`);
if (undoCounts) {
  console.log(`Undo counts: ${JSON.stringify(undoCounts)}`);
}
[inline-code-end]