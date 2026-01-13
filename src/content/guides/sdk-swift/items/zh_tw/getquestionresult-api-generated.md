## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 回應

回傳：[`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetQuestionResult200Response.swift)

## 範例

[inline-code-attrs-start title = 'getQuestionResult 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 下列程式範例仍為測試版本。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let id = "id_example" // String | 

DefaultAPI.getQuestionResult(tenantId: tenantId, id: id) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]