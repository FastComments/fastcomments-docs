## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |
| errorId | string | Yes |  |

## 回應

回傳: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 範例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 範例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-72f3b4';
const templateId: string = 'email_template-9c3a1';
let providedErrorId: string | undefined = undefined; // 可選值，可能在其他地方設定
const errorId: string = providedErrorId ?? 'render_err-5d2f7';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]

---