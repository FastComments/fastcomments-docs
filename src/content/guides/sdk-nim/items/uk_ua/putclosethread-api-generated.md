---
## Параметри

| Назва | Тип | Обов'язкове | Опис |
|------|------|----------|-------------|
| urlId | string | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад використання putCloseThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putCloseThread(urlId = "news/article-title", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Thread closed successfully:", $apiResp
else:
  echo "Failed to close thread, HTTP response:", $httpResponse
[inline-code-end]

---