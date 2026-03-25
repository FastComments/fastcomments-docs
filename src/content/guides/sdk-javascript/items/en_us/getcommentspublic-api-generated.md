
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
const tenantId: string = 'tenant_eu-west_01';
const urlId: string = 'https://www.financialtimes.com/articles/2026/market-update-q1';
const response: GetCommentsPublic200Response = await getCommentsPublic(
  tenantId,
  urlId,
  2,
  undefined,
  'eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.tokenPayload.signature',
  undefined,
  0,
  50,
  5,
  true,
  undefined,
  true,
  false,
  true,
  'en-US',
  'reactions,moderation',
  false,
  true,
  true,
  3,
  false,
  undefined,
  'performance',
  ['feature','fastcomments'],
  'user_9876',
  undefined,
  undefined,
  undefined
);
[inline-code-end]
