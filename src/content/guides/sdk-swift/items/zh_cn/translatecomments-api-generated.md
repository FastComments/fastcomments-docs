将给定的评论翻译为小部件的语言环境。仅在匹配的小部件自定义规则将“评论翻译”设置为自动或手动时可用。

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| sso | string | query | 否 |  |
| locale | string | query | 否 |  |

## 响应

Returns: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CommentTranslationsResponse.swift)

## 示例

[inline-code-attrs-start title = 'translateComments 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于测试阶段。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let commentTranslationsRequest = CommentTranslationsRequest(commentIds: ["commentIds_example"]) // CommentTranslationsRequest | 
let sso = "sso_example" // String |  （可选）
let locale = "locale_example" // String |  （可选）

PublicAPI.translateComments(tenantId: tenantId, urlId: urlId, commentTranslationsRequest: commentTranslationsRequest, options: PublicAPI.TranslateCommentsOptions(sso: sso, locale: locale)) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]