## 參數

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| largeInternalURLSanitized | string | 否 |  |

## 回應

返回: [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## 範例

[inline-code-attrs-start title = 'getGifLarge 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (gifOpt, httpResp) = client.getGifLarge(
  tenantId = "my-tenant-123",
  largeInternalURLSanitized = "https://cdn.example.com/gifs/large123.gif")
if gifOpt.isSome:
  let gif = gifOpt.get()
  echo gif
[inline-code-end]

---