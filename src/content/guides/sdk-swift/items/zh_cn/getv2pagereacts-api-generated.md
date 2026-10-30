## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| sso | string | query | 否 |  |
| includeUserInfo | string | query | 否 | 设置后，返回每个 react id 的用户（头像和用户名）。仅接受 \"public\"，会省略私人资料。 |

## 响应

返回：[`GetV2PageReacts`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/GetV2PageReacts.swift)

## 示例

[inline-code-attrs-start title = 'getV2PageReacts 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let tenantId = "tenantId_example" // String | 
let urlId = "urlId_example" // String | 
let sso = "sso_example" // String |  （可选）
let includeUserInfo = PageReactIncludeUserInfo() // PageReactIncludeUserInfo | 设置后，返回每个 react id 的用户（头像和用户名）。仅接受 \"public\"，会省略私人资料。 （可选）

PublicAPI.getV2PageReacts(tenantId: tenantId, urlId: urlId, options: PublicAPI.GetV2PageReactsOptions(sso: sso, includeUserInfo: includeUserInfo)) { (response, error) in
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