## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|----------|-------------|
| value | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationPageSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_page_search_response.nim)

## 範例

[inline-code-attrs-start title = 'getSearchPages 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchPages(value = "news/politics/election-2024", sso = "sso-user-7f3b9c")
if response.isSome:
  let pageSearch = response.get()
  echo "Moderation page search returned"
else:
  echo "No moderation pages found"
[inline-code-end]

---