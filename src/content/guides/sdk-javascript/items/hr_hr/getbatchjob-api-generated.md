Status, napredak i, kada je završeno, rezultat jednog batch posla.

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vraća: [`GetBatchJobAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobAPIResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getBatchJob primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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