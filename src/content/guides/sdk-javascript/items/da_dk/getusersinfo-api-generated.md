Massebrugerinfo for en lejer. Givet bruger-id'er returneres visningsinfo fra User / SSOUser.  
Brugt af kommentarwidgeten til at berige brugere, der lige er dukket op via en tilstedeværelseshændelse.  
Ingen sidekontekst: privatliv håndhæves ensartet (private profiler maskeres).

## Parameters

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| ids | string | Ja |  |

## Svar

Returnerer: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getUsersInfo Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]