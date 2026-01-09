## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| usernameStartsWith | string | 아니요 |  |
| mentionGroupIds | seq[string] | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## 예제

[inline-code-attrs-start title = 'searchUsers 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  usernameStartsWith = "jo",
  mentionGroupIds = @["editors", "sports-team"],
  sso = "sso-abc-456"
)

if response.isSome:
  let users = response.get()
  echo "Users found: ", users
else:
  echo "No users found; HTTP status: ", httpResponse.status
[inline-code-end]

---