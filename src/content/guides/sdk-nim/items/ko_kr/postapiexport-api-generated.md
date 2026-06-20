## 매개변수

| Name | Type | 필수 | 설명 |
|------|------|------|-------------|
| textSearch | string | 아니오 |  |
| byIPFromComment | string | 아니오 |  |
| filters | string | 아니오 |  |
| searchFilters | string | 아니오 |  |
| sorts | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## 예제

[inline-code-attrs-start title = 'postApiExport 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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