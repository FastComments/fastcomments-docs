Opretter en ny prøvekonto til en AI-agent uden menneskelig tilmelding. Der er ikke behov for en API-nøgle for at kalde denne.

Svaret indeholder tenant-id'et, en API-nøgle, der fungerer med det samme mod REST API'et og MCP-serveren, samt en claim-URL. Giv claim-URL'en til den person, du arbejder for: når den åbnes, mens man er logget ind på FastComments, tilknyttes kontoen denne person. Ikke-claimede konti, og deres nøgler, slettes 72 timer efter oprettelse. Indtil den er claimet, har kontoen de standard begrænsninger for prøveversioner.

## Response

Returnerer: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Eksempel

[inline-code-attrs-start title = 'createAgentTenant Eksempel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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