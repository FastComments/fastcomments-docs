Masovno dobijanje informacija o korisnicima za tenant. Dati userIds, vraća informacije za prikaz iz User / SSOUser.  
Koristi se u widgetu za komentare kako bi obogatio korisnike koji su se upravo pojavili putem događaja prisutnosti.  
Nema konteksta stranice: privatnost se primjenjuje uniformno (privatni profili su maskirani).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| ids | string | No |  |

## Response

Vraća: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Example

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]

---