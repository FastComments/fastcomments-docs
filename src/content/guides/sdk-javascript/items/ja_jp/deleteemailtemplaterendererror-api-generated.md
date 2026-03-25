## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| errorId | string | はい |  |

## レスポンス

返却: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f3b4c2a';
const templateEnvironment: string | undefined = 'production'; // オプションの環境セレクター
const id: string = `emailTemplates/${templateEnvironment ?? 'staging'}/welcome_v2`;
const errorId: string = 'err_5a9d2f1c';
const result: FlagCommentPublic200Response = await deleteEmailTemplateRenderError(tenantId, id, errorId);
console.log(result);
[inline-code-end]

---