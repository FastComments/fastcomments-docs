The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | 예를 들어 migrate-pages와 같은 해당 유형의 작업만. |
| status | string | query | No | 요청됨, 실행 중, 성공 또는 실패 상태인 작업만. |
| skip | integer | query | No | 페이지 매김을 위해 건너뛸 작업 수. |

## Response

Returns: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## 예시

[inline-code-attrs-start title = 'getBatchJobs 예시'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 다음 코드 샘플은 아직 베타 버전입니다. 문제가 있으면 http://github.com/OpenAPITools/openapi-generator/issues/new 로 보고해 주세요
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | 예를 들어 migrate-pages와 같은 해당 유형의 작업만. (옵션)
let status = "status_example" // String | 요청됨, 실행 중, 성공 또는 실패 상태인 작업만. (옵션)
let skip = 987 // Int | 페이지 매김을 위해 건너뛸 작업 수. (옵션)

DefaultAPI.getBatchJobs(tenantId: tenantId, options: DefaultAPI.GetBatchJobsOptions(type: type, status: status, skip: skip)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]