## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| skip | number | Nee |  |

## Respons

Geeft terug: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSSOUsersResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getSSOUsers Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_8f3b2a1c";
const usersWithoutSkip: GetSSOUsersResponse = await getSSOUsers(tenantId);
const skip: number = 50;
const usersWithSkip: GetSSOUsersResponse = await getSSOUsers(tenantId, skip);
[inline-code-end]