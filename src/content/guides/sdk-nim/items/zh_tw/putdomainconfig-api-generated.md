## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenantId | string | 是 |  |
| domainToUpdate | string | 否 |  |
| updateDomainConfigParams | UpdateDomainConfigParams | 否 |  |

## 回應

回傳: [`Option[PutDomainConfigResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_put_domain_config_response.nim)

## 範例

[inline-code-attrs-start title = 'putDomainConfig 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "blog.example.com",
  updateDomainConfigParams = UpdateDomainConfigParams(
    allowAnonymous = false,
    moderationEnabled = true,
    maxCommentLength = 800,
    allowedOrigins = @["https://blog.example.com", "https://cdn.blog.example.com"],
    enableThreadedComments = true
  )
)

if response.isSome:
  let cfg = response.get()
  echo cfg
else:
  echo "Failed to update domain config, HTTP status: ", httpResponse.status
[inline-code-end]

---