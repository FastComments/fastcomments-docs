
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
async function run(): Promise<void> {
  const tenantId: string = 'publisher-135';
  const urlId: string = 'news/2026/05/20/market-update';
  const result: GetCommentsPublic200Response = await getCommentsPublic(
    tenantId,
    urlId,
    1,
    undefined,
    'sso-token-4f2b',
    0,
    0,
    20,
    3,
    true,
    undefined,
    true,
    false,
    true,
    'en-US',
    'reactions,sharing',
    false,
    true,
    true,
    2,
    false,
    undefined,
    'economic outlook',
    ['economy','markets'],
    'reader-452',
    '{"displayMode":"compact"}',
    undefined,
    undefined
  );
  console.log(result);
}
run();
[inline-code-end]
