## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| page | number | No |  |
| limit | number | No |  |
| skip | number | No |  |
| asTree | boolean | No |  |
| skipChildren | number | No |  |
| limitChildren | number | No |  |
| maxTreeDepth | number | No |  |
| urlId | string | No |  |
| userId | string | No |  |
| anonUserId | string | No |  |
| contextUserId | string | No |  |
| hashTag | string | No |  |
| parentId | string | No |  |
| direction | SortDirections | No |  |

## Risposta

Restituisce: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getComments'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // page (pagina)
  20, // limit (limite)
  0, // skip (salto)
  true, // asTree (comeAlbero)
  1, // skipChildren (saltaFigli)
  3, // limitChildren (limiteFigli)
  4, // maxTreeDepth (profonditàMassimaAlbero)
  'articles/2026/new-product-launch', // urlId (idUrl)
  'user_7890', // userId (idUtente)
  'anon_4f3b2', // anonUserId (idUtenteAnonimo)
  undefined, // contextUserId (idUtenteContesto)
  '#launch', // hashTag (hashtag)
  undefined // parentId (idGenitore)
);
[inline-code-end]