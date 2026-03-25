## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createUserBadgeParams | CreateUserBadgeParams | Да |  |

## Ответ

Возвращает: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // необязательные критерии
  displayOrder: 10,
  metadata: { featured: true } // необязательные метаданные
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---