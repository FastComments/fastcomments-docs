## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primer

[inline-code-attrs-start title = 'updateQuestionConfig Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiResp, httpResp) = client.updateQuestionConfig(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionConfigBody = UpdateQuestionConfigBody()
)

if apiResp.isSome:
  let resp = apiResp.get()
[inline-code-end]