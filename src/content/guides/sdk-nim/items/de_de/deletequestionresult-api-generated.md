## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| id | string | Nein |  |

## Antwort

Gibt zurück: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für deleteQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let resultId = "question-result-456"
let (response, httpResponse) = client.deleteQuestionResult(tenantId = tenantId, id = resultId)
if response.isSome:
  let emptyResp = response.get()
  echo "Deleted question result:", resultId
[inline-code-end]

---