## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| usernameStartsWith | string | No |  |
| mentionGroupIds | Array<string> | No |  |
| sso | string | No |  |

## Response

Returns: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/SearchUsers200Response.ts)

## Example

[inline-code-attrs-start title = 'searchUsers Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tnt_7b3f1e9d';
  const urlId: string = 'url_4a2c9f1b';
  const usernameStartsWith: string = 'jo';
  const mentionGroupIds: Array<string> = ['group-8a1f2c3d', 'group-4b3d9e6f'];
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.example.signature';
  const result: SearchUsers200Response = await searchUsers(tenantId, urlId, usernameStartsWith, mentionGroupIds, sso);
  console.log(result);
})();
[inline-code-end]
