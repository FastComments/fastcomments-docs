Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave-informatie van User / SSOUser.
Wordt gebruikt door de commentaarwidget om gebruikers te verrijken die zojuist verschenen via een presence-evenement.
Geen paginacontext: privacy wordt uniform gehandhaafd (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Respons

Retourneert: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // optioneel; als undefined standaard komma gebruiken
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---