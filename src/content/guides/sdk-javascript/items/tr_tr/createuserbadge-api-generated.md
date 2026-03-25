## Parametreler

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| createUserBadgeParams | CreateUserBadgeParams | Yes |  |

## Yanıt

Döndürür: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createUserBadge Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // isteğe bağlı kriterler
  displayOrder: 10,
  metadata: { featured: true } // isteğe bağlı meta veriler
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---