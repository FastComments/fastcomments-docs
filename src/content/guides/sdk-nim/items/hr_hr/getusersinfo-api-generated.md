Masovne informacije o korisnicima za najmoprimca (tenant). Za zadane userIds vraća informacije o prikazu iz User / SSOUser.
Koristi ga widget za komentare kako bi obogatio korisnike koji su upravo pojavili putem presence eventa.
Bez konteksta stranice: privatnost se provodi jednoliko (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Odgovor

Vraća: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

## Primjer

[inline-code-attrs-start title = 'getUsersInfo primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (usersInfoOpt, httpResp) = client.getUsersInfo(tenantId = "my-tenant-123", ids = "user42")
if usersInfoOpt.isSome:
  let usersInfo = usersInfoOpt.get()
[inline-code-end]