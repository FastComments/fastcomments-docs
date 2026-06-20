## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| textSearch | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationSuggestResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_suggest_response.nim)

## 예제

[inline-code-attrs-start title = 'getSearchSuggest 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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