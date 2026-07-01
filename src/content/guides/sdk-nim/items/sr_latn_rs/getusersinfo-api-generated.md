Skup podataka o korisnicima za tenant. Na osnovu userIds, vraća prikazne informacije iz User / SSOUser.  
Koristi se u widžetu za komentare kako bi se obogatili korisnici koji su se upravo pojavili putem presence događaja.  
Bez konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Response

Returns: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]