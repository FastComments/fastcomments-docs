## Parámetros

| Nombre | Tipo | Obligatorio | Descripción |
|------|------|----------|-------------|
| tenantId | string | Sí |  |
| id | string | No |  |
| updatableCommentParams | UpdatableCommentParams | No |  |
| contextUserId | string | No |  |
| doSpamCheck | bool | No |  |
| isLive | bool | No |  |

## Respuesta

Devuelve: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de updateComment'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateComment(
  tenantId = "my-tenant-123",
  id = "cmt-987654",
  updatableCommentParams = UpdatableCommentParams(
    text = "Updated comment: corrected facts and clarified wording.",
    isApproved = true,
    tags = @["news", "update"]
  ),
  contextUserId = "user-456",
  doSpamCheck = true,
  isLive = true
)

if response.isSome:
  let apiResp = response.get()
  discard apiResp
[inline-code-end]

---