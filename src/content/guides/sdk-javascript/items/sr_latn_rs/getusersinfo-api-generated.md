Skupne informacije o korisnicima za tenant. Na osnovu userIds vraća informacije za prikaz iz User / SSOUser.
Koristi widget za komentare da obogati korisnike koji su se upravo pojavili putem presence događaja.
Bez konteksta stranice: privatnost se primenjuje jednako (privatni profili su zamaskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // opciono; ako je undefined, podrazumevano je zarez
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---