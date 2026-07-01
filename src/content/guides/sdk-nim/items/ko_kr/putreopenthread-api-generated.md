## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| urlId | string | 예 |  |
| sso | string = "" | 아니오 |  |

## Response

Returns: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## Example

[inline-code-attrs-start title = 'putReopenThread 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
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