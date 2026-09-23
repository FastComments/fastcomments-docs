---
Liste les pages d'un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.
Nécessite que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.
Les pages qui nécessitent SSO sont filtrées en fonction de l'accès aux groupes de l'utilisateur demandeur.

## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| cursor | string | Non |  |
| limit | number | Non |  |
| q | string | Non |  |
| sortBy | PagesSortBy | Non |  |
| hasComments | boolean | Non |  |

## Réponse

Renvoie : [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPublicPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "page_5";
  const limit: number = 20;
  const query: string = "support";
  const hasComments: boolean = true;

  const response: GetPublicPagesResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    query,
    undefined,
    hasComments
  );

  console.log(response);
}
[inline-code-end]

---