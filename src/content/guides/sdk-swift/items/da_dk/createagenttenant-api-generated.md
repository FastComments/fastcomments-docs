Creates a new trial account for an AI agent without a human signup. No API key is needed to call this.

Den nye prøvekonto for en AI‑agent oprettes uden menneskelig tilmelding. Der er ikke behov for en API‑nøgle for at kalde dette.

The response contains the tenant id, an API key that works immediately against the REST API and the
MCP server, and a claim URL. Give the claim URL to the human you are working for: opening it while
logged in to FastComments attaches the account to them. Unclaimed accounts, and their keys, are
deleted 72 hours after creation. Until claimed, the account has the standard trial limits.

Svaret indeholder lejer‑ID‑et, en API‑nøgle, der fungerer med det samme mod REST‑API‑et og MCP‑serveren, samt en claim‑URL. Giv claim‑URL’en til den person, du arbejder for: at åbne den, mens du er logget ind på FastComments, knytter kontoen til dem. Uclaimede konti og deres nøgler slettes 72 timer efter oprettelse. Indtil den er claimet, har kontoen de standard prøvegrænser.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// De følgende kodeeksempler er stadig beta. For eventuelle problemer, rapporter venligst via http://github.com/OpenAPITools/openapi-generator/issues/new
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