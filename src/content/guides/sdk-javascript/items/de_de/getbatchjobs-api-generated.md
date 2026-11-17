Die Batch-Jobs des Tenants (Importe, Exporte, Seitenmigrationen...), neueste zuerst.

## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| type | string | Nein |  |
| status | string | Nein |  |
| skip | number | Nein |  |

## Antwort

Gibt zurück: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getBatchJobs Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Alle Batch-Jobs abrufen
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Fehlgeschlagene Seitenmigrations-Jobs abrufen, wobei die ersten 10 übersprungen werden
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]