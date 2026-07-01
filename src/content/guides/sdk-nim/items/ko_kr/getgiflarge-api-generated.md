## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| largeInternalURLSanitized | string | 아니오 |  |

## 응답

Returns: [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## 예시

[inline-code-attrs-start title = 'getGifLarge 예시'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (gifOpt, httpResp) = client.getGifLarge(
  tenantId = "my-tenant-123",
  largeInternalURLSanitized = "https://cdn.example.com/gifs/large123.gif")
if gifOpt.isSome:
  let gif = gifOpt.get()
  echo gif
[inline-code-end]

---