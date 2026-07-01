## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| commentId | string | Evet |  |
| sso | string = "" | Hayır |  |

## Yanıt

Döndürür: [`Option[ModerationAPIGetLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_logs_response.nim)

## Örnek

[inline-code-attrs-start title = 'getLogs Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (logsOpt, httpRes) = client.getLogs(tenantId = "my-tenant-123", commentId = "cmt-789", sso = "")
if logsOpt.isSome:
  let logs = logsOpt.get()
  echo logs
[inline-code-end]