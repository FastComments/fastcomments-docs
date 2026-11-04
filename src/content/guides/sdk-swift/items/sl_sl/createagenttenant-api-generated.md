Ustvari nov preizkusni račun za AI agenta brez človeške prijave. Za klic tega ni potreben API ključ.

Odgovor vsebuje ID najemnika (tenant id), API ključ, ki takoj deluje proti REST API in
MCP strežniku, ter URL za prevzem računa. URL za prevzem posredujte človeku, za katerega delate: če ga odprete,
medtem ko ste prijavljeni v FastComments, se račun poveže z njim. Neprevzeti računi in njihovi ključi
se izbrišejo 72 ur po ustvarjanju. Do prevzema ima račun standardne omejitve preizkusnega obdobja.

## Odgovor

Vrne: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Primer

[inline-code-attrs-start title = 'Primer createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Naslednji vzorci kode so še v beta fazi. V primeru težav jih prijavite prek http://github.com/OpenAPITools/openapi-generator/issues/new
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