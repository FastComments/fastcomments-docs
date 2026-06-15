req
tenantId
urlId

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| page | number | Ne |  |
| direction | SortDirections | Ne |  |
| sso | string | Ne |  |
| skip | number | Ne |  |
| skipChildren | number | Ne |  |
| limit | number | Ne |  |
| limitChildren | number | Ne |  |
| countChildren | boolean | Ne |  |
| fetchPageForCommentId | string | Ne |  |
| includeConfig | boolean | Ne |  |
| countAll | boolean | Ne |  |
| includei10n | boolean | Ne |  |
| locale | string | Ne |  |
| modules | string | Ne |  |
| isCrawler | boolean | Ne |  |
| includeNotificationCount | boolean | Ne |  |
| asTree | boolean | Ne |  |
| maxTreeDepth | number | Ne |  |
| useFullTranslationIds | boolean | Ne |  |
| parentId | string | Ne |  |
| searchText | string | Ne |  |
| hashTags | Array<string> | Ne |  |
| userId | string | Ne |  |
| customConfigStr | string | Ne |  |
| afterCommentId | string | Ne |  |
| beforeCommentId | string | Ne |  |

## Odgovor

Vraća: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getCommentsPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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