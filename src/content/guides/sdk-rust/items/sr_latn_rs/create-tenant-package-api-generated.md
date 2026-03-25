## Parametri

| Ime | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Da |  |
| create_tenant_package_body | models::CreateTenantPackageBody | Da |  |

## Odgovor

Vraća: [`CreateTenantPackage200Response`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/create_tenant_package_200_response.rs)

## Primer

[inline-code-attrs-start title = 'create_tenant_package Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run() -> Result<(), Error> {
    let params: CreateTenantPackageParams = CreateTenantPackageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        create_tenant_package_body: models::CreateTenantPackageBody {
            name: "Acme Standard Moderation".to_string(),
            description: Some("Standard moderation package for news and blog sites".to_string()),
            enabled: Some(true),
            custom_config: Some(CustomConfigParameters { max_comment_length: Some(1000), require_moderation: Some(true) }),
            gif_rating: Some(GifRating::PG13),
            image_content_profanity_level: Some(ImageContentProfanityLevel::Moderate),
            tos: Some(TosConfig { require_acceptance: Some(true), tos_url: Some("https://acme.example.com/terms".to_string()) }),
        },
    };
    let response: CreateTenantPackage200Response = create_tenant_package(&configuration, params).await?;
    Ok(())
}
[inline-code-end]

---