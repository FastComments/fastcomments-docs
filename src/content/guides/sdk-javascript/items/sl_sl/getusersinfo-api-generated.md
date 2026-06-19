---
Masovni podatki o uporabnikih za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.
Uporablja widget za komentarje, da obogati uporabnike, ki so se pravkar pojavili prek dogodka prisotnosti.
Brez konteksta strani: zasebnost je enotno zagotavljana (zasebni profili so zamaskirani).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Odgovor

Vrne: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_78f9';
const ids: string = 'user_10234,user_10235,user_10236';
const usersInfo: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
// getUsersInfo zahteva le tenantId in ids; izbirni parametri tukaj niso uporabni.
[inline-code-end]

---