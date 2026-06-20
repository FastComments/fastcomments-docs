## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'deleteQuestionConfig Beispiel'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "")

if response.isSome:
  let deleted = response.get()
  echo "Question config deleted for tenant: ", "my-tenant-123"
else:
  echo "Failed to delete question config"
[inline-code-end]

---