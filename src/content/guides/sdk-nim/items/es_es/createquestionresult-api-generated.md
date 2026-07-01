---
## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| createQuestionResultBody | CreateQuestionResultBody | No |  |

## Respuesta

Devuelve: [`Option[CreateQuestionResultResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_question_result_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'createQuestionResult Ejemplo'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let tenantId = "my-tenant-123"
let body = CreateQuestionResultBody()
let (optResult, httpResp) = client.createQuestionResult(tenantId = tenantId, createQuestionResultBody = body)
if optResult.isSome:
  let result = optResult.get()
  echo result
[inline-code-end]

---