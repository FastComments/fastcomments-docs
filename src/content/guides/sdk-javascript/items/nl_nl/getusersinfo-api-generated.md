Bulk gebruikersinformatie voor een tenant. Gegeven userIds, retourneer weergave‑informatie van User / SSOUser.  
Gebruikt door de commentaarwidget om gebruikers die net verschenen via een presence‑event te verrijken.  
Geen paginacontext: privacy wordt uniform afgedwongen (privéprofielen worden gemaskeerd).

## Parameters

| Naam | Type | Verplicht | Beschrijving |
|------|------|-----------|--------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Response

Returns: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Example

[inline-code-attrs-start title = 'getUsersInfo voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]

---