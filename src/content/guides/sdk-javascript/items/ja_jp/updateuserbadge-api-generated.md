## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| id | string | はい |  |
| updateUserBadgeParams | UpdateUserBadgeParams | はい |  |

## レスポンス

戻り値: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## 例

[inline-code-attrs-start title = 'updateUserBadge の例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_acme_987';
  const id: string = 'badge_top_contributor_42';
  const updateUserBadgeParams: UpdateUserBadgeParams = {
    title: 'Top Contributor',
    description: 'Awarded for reaching 100 high-quality comments',
    color: '#FFD700',
    iconUrl: 'https://cdn.acme.com/badges/top-contributor.svg',
    active: true,
    notifyUsers: true
  } as UpdateUserBadgeParams;
  const result: UpdateUserBadge200Response = await updateUserBadge(tenantId, id, updateUserBadgeParams);
  console.log(result);
})();
[inline-code-end]

---