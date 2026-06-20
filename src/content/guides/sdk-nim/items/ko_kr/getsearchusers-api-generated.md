## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| value | string | 아니오 |  |
| sso | string | 아니오 |  |

## 응답

반환: [`Option[ModerationUserSearchResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_user_search_response.nim)

## 예제

[inline-code-attrs-start title = 'getSearchUsers 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getSearchUsers(value = "john.doe@example.com", sso = "sso-acme-789")
if response.isSome:
  let searchRes = response.get()
  echo "Search result:", searchRes
else:
  echo "No users found"
[inline-code-end]

---