## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |

## Odpowiedź

Zwraca: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Przykład

[inline-code-attrs-start title = 'Przykład getV2PageReacts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactsOpt, httpResp) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactsOpt.isSome:
  let reacts = reactsOpt.get()
  echo reacts
[inline-code-end]