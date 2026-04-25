---
## Parametreler

| Ad | Tip | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| createUserBadgeParams | CreateUserBadgeParams | Evet |  |

## Yanıt

Döndürür: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Örnek

[inline-code-attrs-start title = 'createUserBadge Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // isteğe bağlı parametre
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]

---