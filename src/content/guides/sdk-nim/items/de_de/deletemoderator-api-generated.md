## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |
| sendEmail | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteModerator Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteModerator(tenantId = "my-tenant-123", id = "", sendEmail = "")
if response.isSome:
  let apiEmpty = response.get()
  echo "Moderator deleted successfully for tenant my-tenant-123"
else:
  echo "No response returned; inspect httpResponse"
[inline-code-end]

---