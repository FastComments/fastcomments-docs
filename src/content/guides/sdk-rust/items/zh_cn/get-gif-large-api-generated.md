## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| large_internal_url_sanitized | String | 是 |  |

## 响应

返回: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## 示例

[inline-code-attrs-start title = 'get_gif_large 示例'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetGifLargeParams = GetGifLargeParams {
    tenant_id: "acme-corp-tenant".into(),
    large_internal_url_sanitized: "news/article/gif123".into(),
};

let response: GifGetLargeResponse = get_gif_large(&configuration, params).await?;
[inline-code-end]

---