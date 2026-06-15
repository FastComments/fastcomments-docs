req
tenantId
urlId

## Parametre

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| urlId | string | Ja |  |
| page | number | Nej |  |
| direction | SortDirections | Nej |  |
| sso | string | Nej |  |
| skip | number | Nej |  |
| skipChildren | number | Nej |  |
| limit | number | Nej |  |
| limitChildren | number | Nej |  |
| countChildren | boolean | Nej |  |
| fetchPageForCommentId | string | Nej |  |
| includeConfig | boolean | Nej |  |
| countAll | boolean | Nej |  |
| includei10n | boolean | Nej |  |
| locale | string | Nej |  |
| modules | string | Nej |  |
| isCrawler | boolean | Nej |  |
| includeNotificationCount | boolean | Nej |  |
| asTree | boolean | Nej |  |
| maxTreeDepth | number | Nej |  |
| useFullTranslationIds | boolean | Nej |  |
| parentId | string | Nej |  |
| searchText | string | Nej |  |
| hashTags | Array<string> | Nej |  |
| userId | string | Nej |  |
| customConfigStr | string | Nej |  |
| afterCommentId | string | Nej |  |
| beforeCommentId | string | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'Eksempel på getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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