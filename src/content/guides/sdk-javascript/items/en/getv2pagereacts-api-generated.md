## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| sso | string | No |  |
| includeUserInfo | PageReactIncludeUserInfo | No |  |

## Response

Returns: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetV2PageReacts.ts)

## Example

[inline-code-attrs-start title = 'getV2PageReacts Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'abc1234567890';
const urlId: string = '/blog/my-article';

const result: GetV2PageReacts = await getV2PageReacts(tenantId, urlId, 'sso-token-value', 'basic');
[inline-code-end]
