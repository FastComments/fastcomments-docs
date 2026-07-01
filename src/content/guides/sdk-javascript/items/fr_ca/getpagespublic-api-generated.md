Liste des pages pour un locataire. Utilisé par le client de bureau FChat pour remplir sa liste de salles.  
Exige que `enableFChat` soit vrai dans la configuration personnalisée résolue pour chaque page.  
Les pages nécessitant SSO sont filtrées en fonction de l’accès aux groupes de l’utilisateur demandeur.

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| cursor | string | Non |  |
| limit | number | Non |  |
| q | string | Non |  |
| sortBy | PagesSortBy | Non |  |
| hasComments | boolean | Non |  |

## Réponse

Renvoie : [`GetPagesPublicResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublicResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchPages() {
  const tenantId: string = "tenant_12345";
  const cursor: string = "nextPageToken";
  const limit: number = 20;
  const q: string = "blog";
  const sortBy: PagesSortBy = "createdAt";
  const hasComments: boolean = true;

  const response: GetPagesPublicResponse = await getPagesPublic(
    tenantId,
    cursor,
    limit,
    q,
    sortBy,
    hasComments
  );

  console.log(response);
}
[inline-code-end]