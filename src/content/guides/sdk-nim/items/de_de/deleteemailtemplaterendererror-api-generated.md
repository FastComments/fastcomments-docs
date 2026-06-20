## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|-------------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| errorId | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplateRenderError(tenantId = "my-tenant-123", id = "welcome-email-template", errorId = "err-20250615-01")
if response.isSome:
  let emptyResp = response.get()
  echo "Deleted render error, tenant:", "my-tenant-123"
  echo "HTTP status:", httpResponse.status
else:
  echo "No body returned, HTTP status:", httpResponse.status
[inline-code-end]

---