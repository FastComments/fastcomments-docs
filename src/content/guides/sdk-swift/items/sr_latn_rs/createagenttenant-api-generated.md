Kreira novi probni nalog za AI agenta bez ljudske registracije. Za pozivanje nije potreban API ključ.

Odgovor sadrži ID tenanta, API ključ koji odmah funkcioniše sa REST API-jem i MCP serverom, kao i URL za preuzimanje naloga. Dajte URL za preuzimanje osobi za koju radite: otvaranjem tog URL-a dok je prijavljen na FastComments, nalog se povezuje sa njim. Nepreuzeti nalozi, kao i njihovi ključevi, brišu se 72 sata nakon kreiranja. Dok se ne preuzme, nalog ima standardna ograničenja probnog perioda.

## Odgovor

Vraća: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Primer

[inline-code-attrs-start title = 'createAgentTenant Primer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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