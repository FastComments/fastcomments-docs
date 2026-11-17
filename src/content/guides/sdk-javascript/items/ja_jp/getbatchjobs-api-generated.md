テナントのバッチジョブ(インポート、エクスポート、ページ移行など)を新しい順に返します。

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| type | string | いいえ |  |
| status | string | いいえ |  |
| skip | number | いいえ |  |

## レスポンス

戻り値: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## 例

[inline-code-attrs-start title = 'getBatchJobs の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// すべてのバッチジョブを取得
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// 失敗したページ移行ジョブを、最初の10件をスキップして取得
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]