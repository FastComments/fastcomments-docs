Paketne informacije o uporabnikih za najemnika. Glede na userIds vrne prikazne informacije iz User / SSOUser.
Uporablja se v widgetu za komentarje za obogatitev uporabnikov, ki so se pravkar pojavili preko dogodka prisotnosti.
Brez konteksta strani: zasebnost je dosledno uveljavljena (zasebni profili so zamaskirani).

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| ids | string | Ne |  |

## Odgovor

Vrne: [`Option[PageUsersInfoResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_page_users_info_response.nim)

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