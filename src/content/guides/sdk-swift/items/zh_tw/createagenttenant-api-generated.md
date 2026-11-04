為 AI 代理建立新的試用帳戶，無需人工註冊。呼叫此 API 不需要 API 金鑰。

回應包含租戶 ID、可立即用於 REST API 與 MCP 伺服器的 API 金鑰，以及一個認領 URL。請將認領 URL 提供給您所服務的人員：在登入 FastComments 的狀態下開啟該連結，即可將帳戶綁定至該使用者。未認領的帳戶及其金鑰將在建立後 72 小時刪除。在認領之前，該帳戶受標準試用限制約束。

## 回應

回傳：[`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## 範例

[inline-code-attrs-start title = 'createAgentTenant 範例'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// The following code samples are still beta. For any issue, please report via http://github.com/OpenAPITools/openapi-generator/issues/new
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