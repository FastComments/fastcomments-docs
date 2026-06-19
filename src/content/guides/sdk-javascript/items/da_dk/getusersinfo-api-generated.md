Bulk brugerinfo for en tenant. Givet userIds returneres visningsinformation fra User / SSOUser.
Bruges af kommentar-widget til at berige brugere, der lige er dukket op via en presence-begivenhed.
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
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo kræver kun tenantId og ids; valgfrie parametre er ikke relevante her.
[inline-code-end]

---