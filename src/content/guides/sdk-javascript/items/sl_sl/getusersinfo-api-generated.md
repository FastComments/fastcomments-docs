Skupinske informacije o uporabnikih za tenant. Ob podanih userIds vrne prikazne informacije iz User / SSOUser.
Uporablja ga pripomoček za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili prek dogodka prisotnosti.
Brez konteksta strani: zasebnost se dosledno uveljavlja (zasebni profili so zamaskirani).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vrača: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Primer

[inline-code-attrs-start title = 'getUsersInfo Primer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // optional; if undefined default to comma
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---