## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| largeInternalURLSanitized | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[GifGetLargeResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_gif_get_large_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'getGifLarge उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (gifOpt, httpResp) = client.getGifLarge(
  tenantId = "my-tenant-123",
  largeInternalURLSanitized = "https://cdn.example.com/gifs/large123.gif")
if gifOpt.isSome:
  let gif = gifOpt.get()
  echo gif
[inline-code-end]