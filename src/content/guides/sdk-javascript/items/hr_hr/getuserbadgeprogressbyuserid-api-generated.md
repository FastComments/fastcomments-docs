## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| userId | string | Da |  |

## Odgovor

Vraća: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUserBadgeProgressByUserId'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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