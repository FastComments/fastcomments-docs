## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateQuestionResultBody | UpdateQuestionResultBody | Ne |  |

## Odgovor

Vraća: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Primjer

[inline-code-attrs-start title = 'Primjer updateQuestionResult'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateQuestionResult(
  tenantId = "my-tenant-123",
  id = "question-456",
  updateQuestionResultBody = UpdateQuestionResultBody()
)

if optResp.isSome:
  let resp = optResp.get()
[inline-code-end]

---