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
  const tenantId: string = "tenant_98765";
  const commentId: string = "cmt_54321";
  const editKey: string = "edit_1a2b3c4d";
  const sso: string = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.fake.payload";
  const commentTextBasic: GetCommentText200Response = await getCommentText(tenantId, commentId);
  const commentTextWithAuth: GetCommentText200Response = await getCommentText(tenantId, commentId, editKey, sso);
})();
[inline-code-end]
