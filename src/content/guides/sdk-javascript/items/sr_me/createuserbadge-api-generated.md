## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createUserBadgeParams | CreateUserBadgeParams | Да |  |

## Одговор

Враћа: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Пример

[inline-code-attrs-start title = 'createUserBadge Пример'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // опционални параметар
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]