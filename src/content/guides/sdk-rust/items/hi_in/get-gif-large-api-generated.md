## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| large_internal_url_sanitized | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_gif_large उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetGifLargeParams = GetGifLargeParams {
    tenant_id: "acme-corp-tenant".into(),
    large_internal_url_sanitized: "news/article/gif123".into(),
};

let response: GifGetLargeResponse = get_gif_large(&configuration, params).await?;
[inline-code-end]