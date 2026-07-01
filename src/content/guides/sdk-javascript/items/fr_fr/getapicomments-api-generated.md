## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|--------------|-------------|
| page | number | No |  |
| count | number | No |  |
| textSearch | string | No |  |
| byIPFromComment | string | No |  |
| filters | string | No |  |
| searchFilters | string | No |  |
| sorts | string | No |  |
| demo | boolean | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Response

Renvoie : [`GetApiCommentsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiCommentsResponse.ts)

## Example

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
    "dateDesc",           // tri
    false,                // démo
    "tenant-abc123",      // identifiantLocataire
    "sso-token-xyz"       // sso
  );

  const minimalResult: GetApiCommentsResponse = await getApiComments(undefined, 5);
}
[inline-code-end]