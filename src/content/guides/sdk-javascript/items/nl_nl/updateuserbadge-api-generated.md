## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |
| updateUserBadgeParams | UpdateUserBadgeParams | Ja |  |

## Respons

Retourneert: [`UpdateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserBadge200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'updateUserBadge Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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