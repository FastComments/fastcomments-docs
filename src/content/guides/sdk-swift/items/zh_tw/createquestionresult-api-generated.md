## 參數

| 名稱 | Type | Location | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |

## 回應

回傳：[`CreateQuestionResultResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateQuestionResultResponse.swift)

## 範例

[inline-code-attrs-start title = 'createQuestionResult 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式碼範例仍為測試版。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let createQuestionResultBody = CreateQuestionResultBody(urlId: "urlId_example", value: 123, questionId: "questionId_example", anonUserId: "anonUserId_example", userId: "userId_example", commentId: "commentId_example", meta: [MetaItem(name: "name_example", values: ["values_example"])]) // CreateQuestionResultBody | 

DefaultAPI.createQuestionResult(tenantId: tenantId, createQuestionResultBody: createQuestionResultBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]