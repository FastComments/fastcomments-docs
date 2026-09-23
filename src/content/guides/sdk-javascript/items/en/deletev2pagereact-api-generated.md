## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| id | string | Yes |  |
| sso | string | No |  |

## Response

Returns: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Example

[inline-code-attrs-start title = 'deleteV2PageReact Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runDeleteExamples() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "page_98765";
  const commentId: string = "comment_abcde";

  // Call without optional sso
  const resultWithoutSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId);

  // Call with optional sso
  const ssoToken: string = "sso_token_xyz";
  const resultWithSso: CreateV1PageReact = await deleteV2PageReact(tenantId, urlId, commentId, ssoToken);
}
[inline-code-end]
