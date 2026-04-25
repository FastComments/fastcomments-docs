## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| createUserBadgeParams | CreateUserBadgeParams | Так |  |

## Відповідь

Повертає: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад createUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // необов'язковий параметр
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]

---