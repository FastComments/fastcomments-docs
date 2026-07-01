## Parameters

| Nom | Type | Obligatoire | Description |
|------|------|-------------|-------------|
| tenantId | string | Oui |  |
| urlId | string | Oui |  |
| sso | string = "" | Non |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'Exemple putReopenThread'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (apiRespOpt, httpResp) = client.putReopenThread(
  tenantId = "my-tenant-123",
  urlId = "news/article-title",
  sso = ""
)

if apiRespOpt.isSome:
  let emptyResp = apiRespOpt.get()
  discard
[inline-code-end]

---