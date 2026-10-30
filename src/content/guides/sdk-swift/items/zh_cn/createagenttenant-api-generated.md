创建一个新的 AI 代理试用账户，无需人工注册。调用此接口不需要 API 密钥。

响应中包含租户 ID、可立即用于 REST API 和 MCP 服务器的 API 密钥，以及一个认领 URL。将该认领 URL 提供给您为之工作的人员：在登录 FastComments 后打开该链接即可将账户关联到他们。未认领的账户及其密钥将在创建后 72 小时内被删除。在认领之前，账户拥有标准的试用限制。

## Response

返回：[`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant 示例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 以下代码示例仍处于 beta 版。如有任何问题，请通过 http://github.com/OpenAPITools/openapi-generator/issues/new 报告
import FastCommentsSwift

let createAgentTenantBody = CreateAgentTenantBody(name: "name_example", domains: ["domains_example"], agent: "agent_example") // CreateAgentTenantBody | 

DefaultAPI.createAgentTenant(createAgentTenantBody: createAgentTenantBody) { (response, error) in
    guard error == nil else {
        print(error)
        return
    }

    if (response) {
        dump(response)
    }
}
[inline-code-end]