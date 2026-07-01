## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| url_id | String | Da |  |

## Odgovor

Returns: `GetV1PageLikes`

## Primer

[inline-code-attrs-start title = 'Primer get_v1_page_likes'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetV1PageLikesParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
    };
    let _likes = get_v1_page_likes(configuration, params).await?;
    Ok(())
}
[inline-code-end]