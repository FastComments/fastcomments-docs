Massebrugerinfo for en lejer. Givet userIds returneres visningsinfo fra User / SSOUser.  
Bruges af kommentarfunktionen til at berige brugere, der netop er dukket op via en tilstedeværelseshændelse.  
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Svar

Returnerer: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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