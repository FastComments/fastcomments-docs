---
Liste les pages pour un tenant. Utilisé par le client de bureau FChat pour remplir sa liste de salons.
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui exigent SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

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

Renvoie: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---