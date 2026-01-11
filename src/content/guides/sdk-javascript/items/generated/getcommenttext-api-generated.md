## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| commentId | string | Yes |  |
| editKey | string | No |  |
| sso | string | No |  |

## Response

Returns: [`GetCommentText200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetCommentText200Response.ts)

## Example

[inline-code-attrs-start title = 'getCommentText Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_8a3f4b2c';
  const commentId: string = 'cmt_9f4d3a2b';
  const resultBasic: GetCommentText200Response = await getCommentText(tenantId, commentId);
  const editKey: string = 'EK-4f3b2a1c';
  const sso: string = 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9';
  const resultWithOpts: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
  console.log(resultBasic, resultWithOpts);
})();
[inline-code-end]
