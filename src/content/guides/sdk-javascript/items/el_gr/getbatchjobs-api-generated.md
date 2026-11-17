Οι μαζικές εργασίες (batch jobs) του tenant (εισαγωγές, εξαγωγές, μετακινήσεις σελίδων...), από τις νεότερες προς τις παλαιότερες.

## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| type | string | Όχι |  |
| status | string | Όχι |  |
| skip | number | Όχι |  |

## Απάντηση

Επιστρέφει: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Λήψη όλων των μαζικών εργασιών
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Λήψη αποτυχημένων εργασιών μετακίνησης σελίδων, παρακάμπτοντας τις πρώτες 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]