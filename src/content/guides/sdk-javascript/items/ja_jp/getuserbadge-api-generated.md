## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |

## レスポンス

戻り値: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## 例

[inline-code-attrs-start title = 'getUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_6b8f2a1c';
  const id: string = 'badge_9f3d4b2a';
  const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
  const badge: UserBadge | undefined = response.userBadge;
  const badgeName: string | undefined = badge?.name;
  console.log('Retrieved badge name:', badgeName);
})();
[inline-code-end]

---