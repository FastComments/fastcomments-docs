テナントのバッチジョブ（インポート、エクスポート、ページマイグレーションなど）、新しいものが最初です。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|------|------|
| tenantId | string | query | はい |  |
| type | string | query | いいえ | このタイプのジョブのみ、例: migrate-pages. |
| status | string | query | いいえ | このステータスのジョブのみ: requested, running, success or failed. |
| skip | integer | query | いいえ | ページングのためにスキップするジョブ数. |

## Response

返却: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetBatchJobsAPIResponse.swift)

## Example

[inline-code-attrs-start title = 'getBatchJobs の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題がある場合は、http://github.com/OpenAPITools/openapi-generator/issues/new へ報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let type = "type_example" // String | このタイプのジョブのみ、例: migrate-pages. (optional)
let status = "status_example" // String | このステータスのジョブのみ: requested, running, success or failed. (optional)
let skip = 987 // Int | ページングのためにスキップするジョブ数. (optional)

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