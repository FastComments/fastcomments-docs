Masovne informacije o korisnicima za tenant. Za zadane userIds vraća prikazne informacije iz User / SSOUser.
Koristi widget komentara za obogaćivanje korisnika koji su se upravo pojavili putem događaja prisutnosti.
Bez konteksta stranice: privatnost se primjenjuje jednako (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vraća: [`GetUsersInfo200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfo200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-007';
const userIdsList: string[] = ['user_12a', 'user_34b', 'user_56c'];
const separator: string | undefined = undefined; // neobavezno; ako je undefined, zadana vrijednost je zarez
const ids: string = userIdsList.join(separator ?? ',');
const usersInfo: GetUsersInfo200Response = await getUsersInfo(tenantId, ids);
[inline-code-end]

---