## 參數

| 名稱 | 型別 | 必填 | 說明 |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| large_internal_url_sanitized | String | Yes |  |

## 回應

返回：[`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## 範例

[inline-code-attrs-start title = 'get_gif_large 範例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetGifLargeParams = GetGifLargeParams {
    tenant_id: "acme-corp-tenant".into(),
    large_internal_url_sanitized: "news/article/gif123".into(),
};

let response: GifGetLargeResponse = get_gif_large(&configuration, params).await?;
[inline-code-end]

---