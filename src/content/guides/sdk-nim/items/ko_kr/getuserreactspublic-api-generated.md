## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| postIds | seq[string] | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## 예제

[inline-code-attrs-start title = 'getUserReactsPublic 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getUserReactsPublic(
  tenantId = "my-tenant-123",
  postIds = @["news/article-2026", "blog/opinion-987"],
  sso = ""
)
if response.isSome:
  let reacts = response.get()
  echo "Received user reacts for tenant: ", "my-tenant-123"
[inline-code-end]

---