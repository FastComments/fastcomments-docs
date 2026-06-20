## Parámetros

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | Sí |  |
| spam | bool | No |  |
| permNotSpam | bool | No |  |
| sso | string | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de postSetCommentSpamStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postSetCommentSpamStatus(
  commentId = "cmt-20250619-842",
  spam = false,
  permNotSpam = false,
  sso = ""
)
if response.isSome:
  let apiEmpty = response.get()
  discard apiEmpty
[inline-code-end]

---