Создает новую пробную учетную запись для AI‑агента без регистрации человеком. Для вызова этого метода API‑ключ не требуется.

Ответ содержит идентификатор арендатора, API‑ключ, который сразу же работает с REST API и сервером MCP, а также URL‑адрес для привязки. Передайте URL‑адрес привязки человеку, для которого вы работаете: открытие его, будучи авторизованным в FastComments, привязывает учетную запись к этому человеку. Непривязанные учетные записи и их ключи удаляются через 72 часа после создания. Пока учетная запись не привязана, она имеет стандартные ограничения пробного периода.

## Ответ

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Пример

[inline-code-attrs-start title = 'Пример createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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

---