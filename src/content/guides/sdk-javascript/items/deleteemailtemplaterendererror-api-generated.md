## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## Response

Returns: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Example

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Example'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_7f3b2a";
  const id: string = "email_template_8d1f4c";
  const errorId: string = "render_err_20260108_01";
  const correlationId: string | undefined = undefined; // optional value demonstration
  const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
  console.log(result);
})();
[inline-code-end]
