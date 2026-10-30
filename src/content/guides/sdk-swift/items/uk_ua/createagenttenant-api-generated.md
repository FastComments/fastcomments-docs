Creates a new trial account for an AI agent without a human signup. No API key is needed to call this.

Створює новий пробний обліковий запис для AI‑агента без реєстрації людини. Для виклику цього не потрібен API‑ключ.

The response contains the tenant id, an API key that works immediately against the REST API and the
MCP server, and a claim URL. Give the claim URL to the human you are working for: opening it while
logged in to FastComments attaches the account to them. Unclaimed accounts, and their keys, are
deleted 72 hours after creation. Until claimed, the account has the standard trial limits.

Відповідь містить ідентифікатор орендаря, API‑ключ, який одразу працює з REST API та сервером MCP, а також URL‑адресу для прив’язки. Дайте URL‑адресу прив’язки людині, для якої ви працюєте: відкриття її під час входу в FastComments прив’язує обліковий запис до неї. Неприв’язані облікові записи та їхні ключі видаляються через 72 години після створення. До прив’язки обліковий запис має стандартні пробні обмеження.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant Приклад'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наступні зразки коду все ще у бета-версії. У разі будь‑якої проблеми, будь ласка, повідомте за адресою http://github.com/OpenAPITools/openapi-generator/issues/new
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