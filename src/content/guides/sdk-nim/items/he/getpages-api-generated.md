## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |

## תגובה

מחזיר: [`Option[GetPagesAPIResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_pages_api_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPages'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPages(tenantId = "news-site-456")
if response.isSome:
  let pages = response.get()
  echo "Received pages response: ", pages
else:
  echo "No pages returned. HTTP response: ", httpResponse
[inline-code-end]

---