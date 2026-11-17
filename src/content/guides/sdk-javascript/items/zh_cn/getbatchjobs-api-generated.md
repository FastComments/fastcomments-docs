租户的批量任务（导入、导出、页面迁移等），按最新优先排序。

## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| type | string | 否 |  |
| status | string | 否 |  |
| skip | number | 否 |  |

## 响应

返回：[`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## 示例

[inline-code-attrs-start title = 'getBatchJobs 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// 获取所有批量任务
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// 获取失败的页面迁移任务，跳过前 10 条
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]