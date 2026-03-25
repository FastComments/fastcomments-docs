## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| page | number | Nee |  |
| limit | number | Nee |  |
| skip | number | Nee |  |
| asTree | boolean | Nee |  |
| skipChildren | number | Nee |  |
| limitChildren | number | Nee |  |
| maxTreeDepth | number | Nee |  |
| urlId | string | Nee |  |
| userId | string | Nee |  |
| anonUserId | string | Nee |  |
| contextUserId | string | Nee |  |
| hashTag | string | Nee |  |
| parentId | string | Nee |  |
| direction | SortDirections | Nee |  |

## Respons

Retourneert: [`GetComments200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetComments200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getComments Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_42';
const response: GetComments200Response = await getComments(
  tenantId,
  1, // pagina (page)
  20, // limiet (limit)
  0, // overslaan (skip)
  true, // als boom (asTree)
  1, // kinderen overslaan (skipChildren)
  3, // limiet voor kinderen (limitChildren)
  4, // maximale boomdiepte (maxTreeDepth)
  'articles/2026/new-product-launch', // URL-id (urlId)
  'user_7890', // gebruiker-id (userId)
  'anon_4f3b2', // anonieme gebruiker-id (anonUserId)
  undefined, // context gebruiker-id (contextUserId)
  '#launch', // hashtag (hashTag)
  undefined // ouder-id (parentId)
);
[inline-code-end]