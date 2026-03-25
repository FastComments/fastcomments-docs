## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| createUserBadgeParams | CreateUserBadgeParams | はい |  |

## レスポンス

返却値: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## 例

[inline-code-attrs-start title = 'createUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // オプションの条件
  displayOrder: 10,
  metadata: { featured: true } // オプションのメタデータ
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---