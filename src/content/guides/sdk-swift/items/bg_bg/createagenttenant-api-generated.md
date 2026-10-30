---
Създава нов пробен акаунт за AI агент без човешка регистрация. Не е необходим API ключ, за да се извика това.

Отговорът съдържа идентификатора на наемателя, API ключ, който работи незабавно срещу REST API и MCP сървъра, както и URL за заявяване. Дайте URL‑то за заявяване на човека, за когото работите: отварянето му, докато сте влезли в FastComments, свързва акаунта с него. Незаписаните акаунти и техните ключове се изтриват 72 часа след създаването. Докато не бъде заявен, акаунтът има стандартните пробни ограничения.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'Пример за createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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