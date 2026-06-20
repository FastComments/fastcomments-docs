Grupne informacije o korisnicima za tenant. Za zadane userIds, vraća informacije za prikaz iz User / SSOUser.
Koristi ga widget za komentare da obogati korisnike koji su se upravo pojavili putem događaja prisutnosti.
Bez konteksta stranice: privatnost se dosljedno provodi (privatni profili su zamaskirani).

## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Odgovor

Vraća: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Primjer

[inline-code-attrs-start title = 'getUsersInfo Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user-42,user-87")
if response.isSome:
  let usersInfo = response.get()
  echo "Retrieved users info:", usersInfo
else:
  echo "No users info returned. HTTP status:", httpResponse.status
[inline-code-end]

---