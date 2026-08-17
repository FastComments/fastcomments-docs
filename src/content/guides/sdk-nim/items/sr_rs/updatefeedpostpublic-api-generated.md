## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|------|
| tenantId | string | Yes |  |
| postId | string | No |  |
| updateFeedPostParams | UpdateFeedPostParams | No |  |
| options | UpdateFeedPostPublicOptions | No |  |

## Одговор

Враћа: [`Option[CreateFeedPostResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_feed_post_response.nim)

## Пример

[inline-code-attrs-start title = 'updateFeedPostPublic Primer'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateFeedPostPublic(
  tenantId = "my-tenant-123",
  postId = "post-789",
  updateFeedPostParams = UpdateFeedPostParams(),
  options = UpdateFeedPostPublicOptions()
)

if response.isSome:
  let post = response.get()
[inline-code-end]