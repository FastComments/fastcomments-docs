## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| page | number | Non |  |
| count | number | Non |  |
| textSearch | string | Non |  |
| byIPFromComment | string | Non |  |
| filters | string | Non |  |
| searchFilters | string | Non |  |
| sorts | string | Non |  |
| demo | boolean | Non |  |
| tenantId | string | Non |  |
| sso | string | Non |  |

## Réponse

Retourne : [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function loadComments() {
  const fullResult: GetApiCommentsResponse = await getApiComments(
    2,                     // page
    25,                    // nombre
    "feedback",           // rechercheTexte
    "192.168.1.100",      // parIPDepuisCommentaire
    "approved",           // filtres
    "hasReplies",         // filtresDeRecherche
    "dateDesc",           // tris
    false,                // démonstration
    "tenant-abc123",      // identifiantLocataire
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]