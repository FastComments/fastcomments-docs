## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## レスポンス

戻り値: [`AddPageAPIResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/AddPageAPIResponse.swift)

## 例

[inline-code-attrs-start title = 'addPage の例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下のコードサンプルはまだベータです。問題がある場合は http://github.com/OpenAPITools/openapi-generator/issues/new で報告してください
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createAPIPageData = CreateAPIPageData(accessibleByGroupIds: ["accessibleByGroupIds_example"], rootCommentCount: 123, commentCount: 123, title: "title_example", url: "url_example", urlId: "urlId_example") // CreateAPIPageData | 

DefaultAPI.addPage(tenantId: tenantId, createAPIPageData: createAPIPageData) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]

---