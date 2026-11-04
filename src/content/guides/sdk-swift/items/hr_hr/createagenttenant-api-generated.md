Stvara novi probni račun za AI agenta bez ljudske prijave. Za pozivanje ovoga nije potreban API ključ.

Odgovor sadrži ID najmoprimca (tenant id), API ključ koji odmah radi s REST API-jem i MCP poslužiteljem te URL za preuzimanje računa. Dajte URL za preuzimanje osobi za koju radite: otvaranjem dok ste prijavljeni u FastComments račun se povezuje s njom. Nepreuzeti računi i njihovi ključevi brišu se 72 sata nakon stvaranja. Do preuzimanja račun ima standardna ograničenja probnog razdoblja.

## Odgovor

Vraća: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Primjer

[inline-code-attrs-start title = 'createAgentTenant Primjer'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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