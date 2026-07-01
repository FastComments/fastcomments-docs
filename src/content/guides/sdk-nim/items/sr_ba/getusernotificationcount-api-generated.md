## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| sso | string = "" | Ne |  |

## Odgovor

Vraća: [`Option[GetUserNotificationCountResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_user_notification_count_response.nim)

## Primjer

[inline-code-attrs-start title = 'getUserNotificationCount Primjer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getUserNotificationCount(tenantId = "my-tenant-123", sso = "")
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]

---