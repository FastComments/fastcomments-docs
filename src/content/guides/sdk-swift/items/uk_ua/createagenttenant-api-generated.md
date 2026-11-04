Створює новий пробний обліковий запис для AI-агента без участі людини. Для виклику не потрібен API-ключ.

Відповідь містить ідентифікатор орендаря (tenant id), API-ключ, який одразу працює з REST API та
MCP-сервером, а також URL для підтвердження (claim URL). Передайте цей URL людині, на яку ви працюєте: відкриття
посилання під час входу в FastComments прив'язує обліковий запис до неї. Непідтверджені облікові записи та їхні ключі
видаляються через 72 години після створення. До підтвердження обліковий запис має стандартні обмеження пробного періоду.

## Відповідь

Повертає: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Приклад

[inline-code-attrs-start title = 'Приклад createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Наведені нижче приклади коду все ще є бета-версією. У разі будь-яких проблем, будь ласка, повідомте про них через http://github.com/OpenAPITools/openapi-generator/issues/new
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