Masovne informacije o uporabniku za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.  
Uporablja se v pripomočku za komentarje za obogatitev uporabnikov, ki se pojavijo preko dogodka prisotnosti.  
Ni konteksta strani: zasebnost je enotno uveljavljena (zasebni profili so maskirani).

## Parametri

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Odgovor

Vrne: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]