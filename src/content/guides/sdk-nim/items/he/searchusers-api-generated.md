## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| options | SearchUsersOptions | No |  |

## תגובה

מחזיר: [`Option[SearchUsersResult]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_search_users_result.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמת searchUsers'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.searchUsers(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  options = SearchUsersOptions()
)

if response.isSome:
  let result = response.get()
[inline-code-end]