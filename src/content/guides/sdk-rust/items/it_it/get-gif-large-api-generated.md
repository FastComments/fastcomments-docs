## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenant_id | String | Yes |  |
| large_internal_url_sanitized | String | Yes |  |

## Risposta

Restituisce: [`GifGetLargeResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/gif_get_large_response.rs)

## Esempio

[inline-code-attrs-start title = 'Esempio get_gif_large'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params: GetGifLargeParams = GetGifLargeParams {
    tenant_id: "acme-corp-tenant".into(),
    large_internal_url_sanitized: "news/article/gif123".into(),
};

let response: GifGetLargeResponse = get_gif_large(&configuration, params).await?;
[inline-code-end]