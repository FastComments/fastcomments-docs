## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| commentId | string | Sì |  |
| sso | string = "" | No |  |

## Risposta

Restituisce: [`Option[ModerationAPIGetLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_logs_response.nim)

## Esempio

[inline-code-attrs-start title = 'Esempio getLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (logsOpt, httpRes) = client.getLogs(tenantId = "my-tenant-123", commentId = "cmt-789", sso = "")
if logsOpt.isSome:
  let logs = logsOpt.get()
  echo logs
[inline-code-end]