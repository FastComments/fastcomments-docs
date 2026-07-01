## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| commentId | string | Ja |  |
| sso | string = "" | Nej |  |

## Svar

Returnerer: [`Option[ModerationAPIGetLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_logs_response.nim)

## Eksempel

[inline-code-attrs-start title = 'getLogs Eksempel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (logsOpt, httpRes) = client.getLogs(tenantId = "my-tenant-123", commentId = "cmt-789", sso = "")
if logsOpt.isSome:
  let logs = logsOpt.get()
  echo logs
[inline-code-end]