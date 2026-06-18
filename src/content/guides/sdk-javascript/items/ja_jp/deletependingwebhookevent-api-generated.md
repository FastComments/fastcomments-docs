## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deletePendingWebhookEvent の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_5f8d7a34";
const id: string = "webhook_evt_987654321";
const requestNote: string | undefined = undefined;
const response: FlagCommentPublic200Response = await deletePendingWebhookEvent(tenantId, id);
[inline-code-end]