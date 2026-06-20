## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| batchJobId | string | query | 任意 |  |
| sso | string | query | 任意 |  |

## レスポンス

戻り値: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/ModerationExportStatusResponse.swift)

## 例

[inline-code-attrs-start title = 'getApiExportStatus の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータ版です。問題があれば http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let batchJobId = "batchJobId_example" // String |  (任意)
let sso = "sso_example" // String |  (任意)

ModerationAPI.getApiExportStatus(batchJobId: batchJobId, sso: sso) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]