Креира нови пробни налог за AI агента без људске регистрације. За позив није потребан API кључ.

Одговор садржи ID закупца, API кључ који одмах функционише против REST API‑ја и MCP сервера, као и URL за преузимање. Дайте URL за преузимање особи за коју радите: отварање док сте пријављени у FastComments везује налог за њих. Непреузети налози и њихови кључеви се бришу 72 сата након креирања. До преузимања, налог има стандардна пробна ограничења.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant Пример'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Следећи пример кода је још у бета фази. За било који проблем, молимо пријавите га на http://github.com/OpenAPITools/openapi-generator/issues/new
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