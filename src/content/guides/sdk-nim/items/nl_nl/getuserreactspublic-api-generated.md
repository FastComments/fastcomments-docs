## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | seq[string] | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`Option[UserReactsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_user_reacts_response.nim)

## Voorbeeld

[inline-code-attrs-start title = 'getUserReactsPublic Voorbeeld'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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