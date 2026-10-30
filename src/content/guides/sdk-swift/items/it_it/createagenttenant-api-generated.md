---
Crea un nuovo account di prova per un agente AI senza la registrazione umana. Non è necessaria alcuna chiave API per chiamare questo.

La risposta contiene l'ID del tenant, una chiave API che funziona immediatamente contro la REST API e il server MCP, e un URL di claim. Fornisci l'URL di claim all'umano per cui stai lavorando: aprirlo mentre sei connesso a FastComments associa l'account a loro. Gli account non reclamati, e le loro chiavi, vengono eliminati 72 ore dopo la creazione. Fino a quando non vengono reclamati, l'account ha i limiti standard di prova.

## Risposta

Restituisce: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Esempio

[inline-code-attrs-start title = 'Esempio createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// I seguenti esempi di codice sono ancora in beta. Per qualsiasi problema, si prega di segnalare tramite http://github.com/OpenAPITools/openapi-generator/issues/new
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