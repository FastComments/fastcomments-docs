## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| userId | string | Ja |  |

## Antwort

Gibt zurück: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Beispiel

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_7f9c2d3b';
const maybeUserId: string | undefined = 'user_4b8e1f9a'; // optionale Quelle (kann undefined sein)
const userId: string = maybeUserId ?? 'user_fallback0001';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);
console.log(result);
[inline-code-end]

---