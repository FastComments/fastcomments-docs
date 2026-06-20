---
## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| textSearch | string | לא |  |
| byIPFromComment | string | לא |  |
| filters | string | לא |  |
| searchFilters | string | לא |  |
| sorts | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postApiExport'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postApiExport(
  textSearch = "offensive language and spam",
  byIPFromComment = "203.0.113.45",
  filters = "{\"status\":\"pending\",\"severity\":\"high\"}",
  searchFilters = "authorEmail:editor@news-site.com",
  sorts = "-createdAt",
  sso = "sso-session-token-9f8b7c"
)
if response.isSome:
  let exportResp = response.get()
  echo "Moderation export received:", exportResp
else:
  echo "No export returned, HTTP status:", httpResponse.status.code
[inline-code-end]

---