## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | No |  |

## Réponse

Retourne : [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple updateQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.updateQuestionConfig(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionConfigBody = UpdateQuestionConfigBody()
)

if apiResp.isSome:
  let resp = apiResp.get()
[inline-code-end]