## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| domainToUpdate | string | いいえ |  |
| updateDomainConfigParams | UpdateDomainConfigParams | いいえ |  |

## レスポンス

戻り値: [`Option[GetDomainConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_domain_config200response.nim)

## 例

[inline-code-attrs-start title = 'putDomainConfig の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let updateParams = UpdateDomainConfigParams(
  allowAnonymous = false,
  moderationEnabled = true,
  allowedOrigins = @["https://news.example.com"],
  maxCommentLength = 2000
)

let (response, httpResponse) = client.putDomainConfig(
  tenantId = "my-tenant-123",
  domainToUpdate = "news/example-article",
  updateDomainConfigParams = updateParams
)

if response.isSome:
  let domainCfg = response.get()
  discard domainCfg
[inline-code-end]

---