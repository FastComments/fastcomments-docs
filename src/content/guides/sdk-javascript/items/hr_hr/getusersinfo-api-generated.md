Skupne informacije o korisnicima za tenant. Za dane userIds vraća prikazne informacije iz User / SSOUser.
Koristi widget komentara kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti.
Bez konteksta stranice: privatnost se provodi jednako (privatni profili su zamaskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo zahtijeva samo tenantId i ids; opcionalni parametri ovdje nisu primjenjivi.
[inline-code-end]