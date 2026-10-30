Erstellt ein neues Testkonto für einen KI‑Agenten ohne menschliche Anmeldung. Es wird kein API‑Schlüssel benötigt, um dies aufzurufen.

Die Antwort enthält die Mandanten‑ID, einen API‑Schlüssel, der sofort gegen die REST‑API und den MCP‑Server funktioniert, sowie eine Claim‑URL. Geben Sie die Claim‑URL an die Person weiter, für die Sie arbeiten: Das Öffnen der URL, während Sie bei FastComments angemeldet sind, verknüpft das Konto mit ihr. Nicht beanspruchte Konten und deren Schlüssel werden 72 Stunden nach Erstellung gelöscht. Bis zur Beanspruchung hat das Konto die üblichen Testlimits.

## Response

Rückgabe: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Example

[inline-code-attrs-start title = 'createAgentTenant Beispiel'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Die folgenden Codebeispiele sind noch in der Beta-Phase. Bei Problemen bitte melden über http://github.com/OpenAPITools/openapi-generator/issues/new
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