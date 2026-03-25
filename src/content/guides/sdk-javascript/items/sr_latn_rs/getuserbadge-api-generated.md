## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vraća: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadge'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_acme_01";
const id: string = "badge_8c7d2f";
const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
[inline-code-end]

---