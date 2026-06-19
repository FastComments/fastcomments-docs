## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| errorId | string | はい |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## 例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-7f3a2b';
const templateId: string = 'tmpl-9c3e1a2b';
const errorId: string = 'err-2026-06-19-001';
const result: APIEmptyResponse = await deleteEmailTemplateRenderError(tenantId, templateId, errorId);
[inline-code-end]

---