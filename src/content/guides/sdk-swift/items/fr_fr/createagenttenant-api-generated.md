Crée un nouveau compte d'essai pour un agent IA sans inscription humaine. Aucune clé API n'est nécessaire pour appeler ceci.

La réponse contient l'identifiant du tenant, une clé API qui fonctionne immédiatement avec l'API REST et le
serveur MCP, ainsi qu'une URL de revendication. Donnez l'URL de revendication à l'humain pour lequel vous travaillez : l'ouvrir
tout en étant connecté à FastComments rattache le compte à cette personne. Les comptes non revendiqués, ainsi que leurs clés, sont
supprimés 72 heures après leur création. Tant que le compte n'est pas revendiqué, il est soumis aux limites standard de l'essai.

## Réponse

Retourne : [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-swift/blob/main/client/FastCommentsSwift/Models/CreateAgentTenantResponse.swift)

## Exemple

[inline-code-attrs-start title = 'Exemple createAgentTenant'; type = 'swift'; isFunctional = false; inline-code-attrs-end]
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