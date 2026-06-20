## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| textSearch | string | 否 |  |
| sso | string | 否 |  |

## 回應

回傳: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## 範例

[inline-code-attrs-start title = 'getSearchSuggest 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchSuggest(textSearch = "suspicious comment with spammy links", sso = "sso-user-789")
if response.isSome:
  let suggest = response.get()
  echo "ModerationSuggestResponse:"
  echo suggest
else:
  echo "No moderation suggestions returned. HTTP status: ", httpResponse.status
[inline-code-end]

---