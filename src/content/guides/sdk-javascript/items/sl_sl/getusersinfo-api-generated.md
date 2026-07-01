Masovni podatki o uporabniku za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.  
Uporablja vtičnik za komentarje za obogatitev uporabnikov, ki se so pravkar pojavili prek dogodka prisotnosti.  
Ni konteksta strani: zasebnost se izvaja enotno (zasebni profili so maskirani).

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| ids | string | Yes |  |

## Response

Vrne: [`GetUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant";
const ids: string = "user-1001,user-1002";

const usersInfo: GetUsersInfoResponse = await getUsersInfo(tenantId, ids);

// Optional fields in the response may be undefined
const firstUser: PageUserEntry | undefined = usersInfo?.users?.[0];
[inline-code-end]