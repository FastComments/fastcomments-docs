Informacije o množičnih uporabnikih za najemnika. Ob podanih userIds vrne prikazne informacije iz User / SSOUser.
Uporablja widget za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili prek prisotnostnega dogodka.
Brez konteksta strani: zasebnost se uveljavlja enotno (zasebni profili so maskirani).

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Da |  |

## Odgovor

Vrne: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersInfoResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchUsersInfo(): Promise<void> {
  const tenantId: string = "tenant_12345";
  const ids: string = "user_001,user_002";
  const response: PageUsersInfoResponse = await getUsersInfo(tenantId, ids);
  console.log(response);
}
fetchUsersInfo();
[inline-code-end]