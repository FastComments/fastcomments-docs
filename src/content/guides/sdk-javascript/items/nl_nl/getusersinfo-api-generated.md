Bulkgebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaar-widget om gebruikers te verrijken die net zijn verschenen via een aanwezigheidsevenement.
Geen paginacontext: privacy wordt uniform gehandhaafd (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|---------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Respons

Retourneert: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getUsersInfo Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo vereist alleen tenantId en ids; optionele parameters zijn hier niet van toepassing.
[inline-code-end]

---