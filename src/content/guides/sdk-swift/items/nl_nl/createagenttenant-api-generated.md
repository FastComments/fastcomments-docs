---
Maakt een nieuw proefaccount aan voor een AI‑agent zonder menselijke registratie. Er is geen API‑sleutel nodig om dit aan te roepen.

De respons bevat de tenant‑id, een API‑sleutel die onmiddellijk werkt tegen de REST‑API en de MCP‑server, en een claim‑URL. Geef de claim‑URL aan de persoon waarvoor je werkt: deze openen terwijl je bent aangemeld bij FastComments koppelt het account aan hen. Niet‑geclaimde accounts, en hun sleutels, worden 72 uur na creatie verwijderd. Totdat ze geclaimd zijn, heeft het account de standaard proeflimieten.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant Voorbeeld'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De volgende codevoorbeelden zijn nog in de bèta. Meld eventuele problemen via http://github.com/OpenAPITools/openapi-generator/issues/new
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