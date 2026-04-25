## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7a1d2f9b";
const id: string = "email_template_42b1";
const errorId: string = "render_err_2026-04-24_7f3c";
const includeStackTrace: boolean | undefined = undefined; // 可選旗標範例

const response: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
// 若支援可選的 options 物件，它可能看起來像：
 // await deleteEmailTemplateRenderError(tenantId, id, errorId /*, { includeStackTrace } */);
[inline-code-end]