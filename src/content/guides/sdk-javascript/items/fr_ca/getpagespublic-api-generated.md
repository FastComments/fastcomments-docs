Liste des pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Requiert que `enableFChat` soit true dans la configuration personnalisée résolue pour chaque page.
Les pages qui exigent SSO sont filtrées selon l'accès aux groupes de l'utilisateur demandeur.

## Paramètres

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| cursor | string | Non |  |
| limit | number | Non |  |
| q | string | Non |  |
| sortBy | PagesSortBy | Non |  |
| hasComments | boolean | Non |  |

## Réponse

Renvoie: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---