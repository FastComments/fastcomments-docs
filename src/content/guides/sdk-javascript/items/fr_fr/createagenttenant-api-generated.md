Crée un nouveau compte d'essai pour un agent IA sans inscription humaine. Aucune clé API n'est nécessaire pour appeler cette fonction.

La réponse contient l'identifiant du locataire, une clé API qui fonctionne immédiatement avec l'API REST et le serveur MCP, ainsi qu'une URL de réclamation. Donnez l'URL de réclamation à la personne pour laquelle vous travaillez : l'ouvrir tout en étant connecté à FastComments associe le compte à cette personne. Les comptes non réclamés, ainsi que leurs clés, sont supprimés 72 heures après leur création. Jusqu'à ce qu'ils soient réclamés, le compte bénéficie des limites d'essai standard.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| createAgentTenantBody | CreateAgentTenantBody | Oui |  |

## Réponse

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateAgentTenantResponse.ts)

## Exemple

[inline-code-attrs-start title = 'createAgentTenant Exemple'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantBody: CreateAgentTenantBody = {
  name: "Acme Corp",
  domain: "acme.example.com",
  contactEmail: "admin@acme.example.com", // optional
  planId: 3 // optional
};

const result: CreateAgentTenantResponse = await createAgentTenant(tenantBody);
[inline-code-end]