Masovni podaci o korisnicima za tenant. Za zadate userIds, vraća prikazne informacije iz User / SSOUser.
Koristi comment widget da obogati korisnike koji su se upravo pojavili putem događaja prisutnosti.
Nema konteksta stranice: privatnost se primenjuje jednako (privatni profili su maskirani).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Response

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'getUsersInfo Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo zahteva samo tenantId i ids; neobavezni parametri se ovde ne primenjuju.
[inline-code-end]

---