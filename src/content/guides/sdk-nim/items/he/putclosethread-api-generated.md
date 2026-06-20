## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| urlId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-putCloseThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putCloseThread(urlId = "news/article-title", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Thread closed successfully:", $apiResp
else:
  echo "Failed to close thread, HTTP response:", $httpResponse
[inline-code-end]

---