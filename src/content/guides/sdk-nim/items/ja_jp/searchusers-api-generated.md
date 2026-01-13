## パラメータ

| Name | Type | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| urlId | string | はい |  |
| usernameStartsWith | string | いいえ |  |
| mentionGroupIds | seq[string] | いいえ |  |
| sso | string | いいえ |  |

## レスポンス

戻り値: [`Option[SearchUsers_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users200response.nim)

## 例

[inline-code-attrs-start title = 'searchUsers の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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