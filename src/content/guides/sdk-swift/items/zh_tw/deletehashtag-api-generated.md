## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tag | string | path | 是 |  |
| tenantId | string | query | 否 |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/FlagCommentPublic200Response.swift)

## 範例

[inline-code-attrs-start title = 'deleteHashTag 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下程式範例仍為 beta。若有任何問題，請透過 http://github.com/OpenAPITools/openapi-generator/issues/new 回報
import FastCommentsSwift

let tag = "tag_example" // String | 
let tenantId = "tenantId_example" // String |  (可選)
let deleteHashTagRequest = DeleteHashTag_request(tenantId: "tenantId_example") // DeleteHashTagRequest |  (可選)

DefaultAPI.deleteHashTag(tag: tag, tenantId: tenantId, deleteHashTagRequest: deleteHashTagRequest) { (response, error) in
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