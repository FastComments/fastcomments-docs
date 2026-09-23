## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## レスポンス

返り値: [`APIGetUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeResponse.ts)

## 例

[inline-code-attrs-start title = 'getUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoGetUserBadge() {
    const tenantId: string = "tenant-9f8b7c6d";
    const userId: string = "user-123456";
    const badgeResponse: APIGetUserBadgeResponse = await getUserBadge(tenantId, userId);
}
[inline-code-end]