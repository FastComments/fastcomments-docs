---
Masovne informacije o korisnicima za tenant. Za zadate userIds, vraća prikazne informacije iz User / SSOUser.
Koristi se u widgetu za komentare da obogati korisnike koji su se upravo pojavili putem presence event-a.
Bez konteksta stranice: privatnost se primenjuje uniformno (privatni profili su maskirani).

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Odgovor

Vraća: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getUsersInfo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---