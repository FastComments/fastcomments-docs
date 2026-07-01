## Parametry

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| id | string | No |  |

## Odpowiedź

Zwraca: [`Option[APIGetCommentResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_get_comment_response.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getComment(tenantId = "my-tenant-123", id = "cmt-789")
if response.isSome:
  let comment = response.get()
  discard comment
[inline-code-end]