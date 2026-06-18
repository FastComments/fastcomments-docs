## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## 例

[inline-code-attrs-start title = 'deleteNotificationCount の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_5f3d2e1b';
const id: string = 'notification_9a4b1c2';
const result: FlagCommentPublic200Response = await deleteNotificationCount(tenantId, id);
[inline-code-end]

---