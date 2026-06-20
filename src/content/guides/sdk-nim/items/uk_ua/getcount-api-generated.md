## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| textSearch | string | Ні |  |
| byIPFromComment | string | Ні |  |
| filter | string | Ні |  |
| searchFilters | string | Ні |  |
| demo | bool | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[ModerationAPICountCommentsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_count_comments_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getCount'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCount(
  textSearch = "climate change",
  byIPFromComment = "203.0.113.5",
  filter = "status:approved",
  searchFilters = "author:john.doe@example.com;tag:opinion",
  demo = false,
  sso = "sso_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9"
)
if response.isSome:
  let countResp = response.get()
  discard countResp
  echo "Count response received"
else:
  echo "No count data returned"
[inline-code-end]

---