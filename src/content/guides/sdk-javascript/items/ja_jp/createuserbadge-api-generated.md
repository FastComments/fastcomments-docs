## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createUserBadgeParams | CreateUserBadgeParams | はい |  |

## レスポンス

戻り値: [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APICreateUserBadgeResponse.ts)

## 例

[inline-code-attrs-start title = 'createUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_84a2c6b2';
  const createUserBadgeParams: CreateUserBadgeParams = {
    name: 'Early Supporter',
    description: 'Awarded to users who joined during the alpha launch',
    iconUrl: 'https://cdn.fastcomments.com/badges/early-supporter.png',
    criteria: 'Joined before 2021-06-01',
    isActive: true,
    notifyUsers: true // オプションのパラメータ
  };
  const result: APICreateUserBadgeResponse = await createUserBadge(tenantId, createUserBadgeParams);
  console.log(result);
})();
[inline-code-end]

---