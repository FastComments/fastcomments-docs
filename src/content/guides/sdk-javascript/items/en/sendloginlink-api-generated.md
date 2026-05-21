## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| redirectURL | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'sendLoginLink Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_93';
  const id: string = 'user_72f9b';
  const redirectURL: string = 'https://app.mycompany.com/welcome';
  const result: FlagCommentPublic200Response = await sendLoginLink(tenantId, id, redirectURL);
  console.log(result);
})();
[inline-code-end]
