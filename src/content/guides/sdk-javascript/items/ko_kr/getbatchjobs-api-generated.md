테넌트의 배치 작업(가져오기, 내보내기, 페이지 마이그레이션 등), 최신 항목부터 표시됩니다.

## 매개변수

| 이름 | 타입 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| type | string | 아니요 |  |
| status | string | 아니요 |  |
| skip | number | 아니요 |  |

## 응답

반환값: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'getBatchJobs 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// 모든 배치 작업 가져오기
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// 실패한 페이지 마이그레이션 작업 가져오기, 처음 10개 건너뛰기
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]