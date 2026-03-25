## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Ja |  |

## Antwoord

Retourneert: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUserBadgeProgressById Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const optionalTenantSuffix: string | undefined = undefined;
const tenantId: string = `5f8d0d55-1234-4ab1-9e2a-3f2b5c6d7e8f${optionalTenantSuffix ?? ''}`;
const id: string = '3a2b1c4d-5678-4ef0-9abc-def123456789';
const result: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, id);
[inline-code-end]

---