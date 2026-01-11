
req
tenantId
urlId

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| page | number | No |  |
| direction | SortDirections | No |  |
| sso | string | No |  |
| skip | number | No |  |
| skipChildren | number | No |  |
| limit | number | No |  |
| limitChildren | number | No |  |
| countChildren | boolean | No |  |
| fetchPageForCommentId | string | No |  |
| includeConfig | boolean | No |  |
| countAll | boolean | No |  |
| includei10n | boolean | No |  |
| locale | string | No |  |
| modules | string | No |  |
| isCrawler | boolean | No |  |
| includeNotificationCount | boolean | No |  |
| asTree | boolean | No |  |
| maxTreeDepth | number | No |  |
| useFullTranslationIds | boolean | No |  |
| parentId | string | No |  |
| searchText | string | No |  |
| hashTags | Array<string> | No |  |
| userId | string | No |  |
| customConfigStr | string | No |  |
| afterCommentId | string | No |  |
| beforeCommentId | string | No |  |

## Response

Returns: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentsPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'getCommentsPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_001';
const urlId: string = 'article/2025/launch';
const page: number = 1;
const direction: SortDirections = 'newest' as unknown as SortDirections;
const limit: number = 25;
const limitChildren: number = 3;
const countChildren: boolean = true;
const includeConfig: boolean = true;
const response: GetCommentsPublic200Response = await getCommentsPublic(tenantId, urlId, page, direction, undefined, 0, 0, limit, limitChildren, countChildren, undefined, includeConfig);
[inline-code-end]
