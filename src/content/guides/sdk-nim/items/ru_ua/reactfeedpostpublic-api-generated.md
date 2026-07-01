## Параметры

| Имя | Тип | Обязательно | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| postId | string | Нет |  |
| reactBodyParams | ReactBodyParams | Нет |  |
| options | ReactFeedPostPublicOptions | Нет |  |

## Ответ

Возвращает: [`Option[ReactFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_react_feed_post_response.nim)

## Пример

[inline-code-attrs-start title = 'reactFeedPostPublic Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let reactParams = ReactBodyParams()
let (optResp, httpResp) = client.reactFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-456",
  reactBodyParams = reactParams,
  options = ReactFeedPostPublicOptions()
)
if optResp.isSome:
  let resp = optResp.get()
  echo resp
[inline-code-end]