Tworzy nowe konto próbne dla agenta AI bez rejestracji przez człowieka. Do wywołania tego endpointu nie jest potrzebny klucz API.

Odpowiedź zawiera identyfikator tenanta, klucz API działający natychmiast z REST API i serwerem MCP oraz URL do przejęcia konta (claim URL). Przekaż claim URL człowiekowi, dla którego pracujesz: otwarcie go będąc zalogowanym do FastComments przypisuje konto do tej osoby. Nieprzejęte konta oraz ich klucze są usuwane 72 godziny po utworzeniu. Do momentu przejęcia konto ma standardowe limity wersji próbnej.

## Odpowiedź

Zwraca: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Przykład

[inline-code-attrs-start title = 'Przykład createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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