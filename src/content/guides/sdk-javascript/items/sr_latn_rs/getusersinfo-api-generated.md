Bulk informacije o korisnicima za tenant. Dati userIds, vraća prikazne informacije iz User / SSOUser.  
Koristi se u widgetu za komentare da obogati korisnike koji su se upravo pojavili putem događaja prisustva.  
Nema konteksta stranice: privatnost se primenjuje jednako (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
```typescript
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Opcionalna polja u odgovoru mogu biti neodređena
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
```
[inline-code-end]