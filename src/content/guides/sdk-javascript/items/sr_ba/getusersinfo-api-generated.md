---
Grupne informacije o korisnicima za tenant. Za navedene userIds, vraća prikazne informacije iz User / SSOUser.
Koristi se u widgetu za komentare da obogati korisnike koji su se upravo pojavili putem događaja prisutnosti.
Nema konteksta stranice: privatnost se provodi jednako (privatni profili su zamaskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getUsersInfo Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo only requires tenantId and ids; optional parameters are not applicable here.
[inline-code-end]

---