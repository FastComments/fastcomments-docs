## Parámetros

| Nombre | Tipo | Requerido | Descripción |
|------|------|----------|-------------|
| userId | string | No |  |
| direction | SortDirections | No |  |
| repliesToUserId | string | No |  |
| page | float64 | No |  |
| includei10n | bool | No |  |
| locale | string | No |  |
| isCrawler | bool | No |  |

## Respuesta

Devuelve: [`Option[GetCommentsForUserResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comments_for_user_response.nim)

## Ejemplo

[inline-code-attrs-start title = 'Ejemplo de getCommentsForUser'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentsForUser(
  userId = "user-8421",
  direction = SortDirections.Newest,
  repliesToUserId = "",
  page = 1.0,
  includei10n = true,
  locale = "en-US",
  isCrawler = false
)

if response.isSome:
  let comments = response.get()
  discard comments
[inline-code-end]

---