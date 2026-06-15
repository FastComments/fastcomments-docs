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
const tenantId: string = 'acme-inc-tenant-01';
const userId: string = 'user_73c9b2';
const progress: GetUserBadgeProgressById200Response = await getUserBadgeProgressByUserId(tenantId, userId);

async function maybeFetchProgress(tenant: string, user?: string): Promise<GetUserBadgeProgressById200Response | null> {
  if (!user) return null;
  return await getUserBadgeProgressByUserId(tenant, user);
}

const optionalResult: GetUserBadgeProgressById200Response | null = await maybeFetchProgress(tenantId, userId);
[inline-code-end]

---