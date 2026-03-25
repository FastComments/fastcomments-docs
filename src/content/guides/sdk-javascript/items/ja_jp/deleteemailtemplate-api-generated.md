## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteEmailTemplate の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f4c9d1e';
const templateId: string = 'tmpl_welcome_2024-03';
const notifyAdmin: boolean | undefined = true; // オプションのパラメータの例

const result: FlagCommentPublic200Response = await deleteEmailTemplate(tenantId, templateId);
[inline-code-end]

---