## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filters | string | Ні |  |
| searchFilters | string | Ні |  |
| afterId | string | Ні |  |
| demo | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[ModerationAPIGetCommentIdsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_comment_ids_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiIds'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getApiIds(
  textSearch = "urgent moderation review",
  byIPFromComment = "203.0.113.45",
  filters = "status:pending,flagged",
  searchFilters = "author:jane.doe@example.com",
  afterId = "cmt_9f8e7d6a",
  demo = false,
  sso = "sso-token-6b7f9a"
)

if response.isSome:
  let idsResp = response.get()
  echo idsResp
[inline-code-end]

---