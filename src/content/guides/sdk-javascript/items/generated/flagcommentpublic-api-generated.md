## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| isFlagged | boolean | Yes |  |
| sso | string | No |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'flagCommentPublic Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoFlagComment(): Promise<void> {
  const tenantId: string = 'tenant_9a7c3f2b';
  const commentId: string = 'cmt_5d8e12a4';
  const isFlagged: boolean = true;
  const sso: string | undefined = 'sso_tok_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const result: FlagCommentPublic200Response = await flagCommentPublic(tenantId, commentId, isFlagged, sso);
  console.log(result);
}
demoFlagComment();
[inline-code-end]
