req
tenantId
urlId

## Paramètres

| Name | Type | Obligatoire | Description |
|------|------|------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| page | number | Non |  |
| direction | SortDirections | Non |  |
| sso | string | Non |  |
| skip | number | Non |  |
| skipChildren | number | Non |  |
| limit | number | Non |  |
| limitChildren | number | Non |  |
| countChildren | boolean | Non |  |
| fetchPageForCommentId | string | Non |  |
| includeConfig | boolean | Non |  |
| countAll | boolean | Non |  |
| includei10n | boolean | Non |  |
| locale | string | Non |  |
| modules | string | Non |  |
| isCrawler | boolean | Non |  |
| includeNotificationCount | boolean | Non |  |
| asTree | boolean | Non |  |
| maxTreeDepth | number | Non |  |
| useFullTranslationIds | boolean | Non |  |
| parentId | string | Non |  |
| searchText | string | Non |  |
| hashTags | Array<string> | Non |  |
| userId | string | Non |  |
| customConfigStr | string | Non |  |
| afterCommentId | string | Non |  |
| beforeCommentId | string | Non |  |

## Réponse

Renvoie: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-news';
const urlId: string = '/articles/2026/fastcomments-update';
const page: number = 1;
const skip: number = 0;
const limit: number = 25;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const result: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  page,
  undefined,
  undefined,
  skip,
  undefined,
  limit,
  undefined,
  countChildren,
  undefined,
  includeConfig
);
[inline-code-end]

---