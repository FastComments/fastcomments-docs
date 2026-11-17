租戶的批次工作(匯入、匯出、頁面遷移等),按最新排列在前。

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| type | string | 否 |  |
| status | string | 否 |  |
| skip | number | 否 |  |

## 回應

回傳:[`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## 範例

[inline-code-attrs-start title = 'getBatchJobs 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// 取得所有批次工作
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// 取得失敗的頁面遷移工作,略過前 10 筆
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]