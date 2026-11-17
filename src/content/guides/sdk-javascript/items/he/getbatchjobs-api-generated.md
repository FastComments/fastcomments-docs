עבודות האצווה של הדייר (ייבוא, ייצוא, העברות דפים...), מהחדש לישן.

## פרמטרים

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| type | string | No |  |
| status | string | No |  |
| skip | number | No |  |

## תשובה

מחזיר: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// קבלת כל עבודות האצווה
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// קבלת עבודות העברת דפים שנכשלו, תוך דילוג על 10 הראשונות
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]