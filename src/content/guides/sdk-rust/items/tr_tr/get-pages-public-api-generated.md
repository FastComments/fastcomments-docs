Bir kiracı (tenant) için sayfaları listeler. FChat masaüstü istemcisi tarafından oda listesini doldurmak için kullanılır.
Her sayfa için çözümlenen özel yapılandırmada `enableFChat` değerinin true olması gerekir.
SSO gerektiren sayfalar, istekte bulunan kullanıcının grup erişimine göre filtrelenir.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| cursor | String | Hayır |  |
| limit | i32 | Hayır |  |
| q | String | Hayır |  |
| sort_by | models::PagesSortBy | Hayır |  |
| has_comments | bool | Hayır |  |

## Yanıt

Döndürür: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## Örnek

[inline-code-attrs-start title = 'get_pages_public Örneği'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]