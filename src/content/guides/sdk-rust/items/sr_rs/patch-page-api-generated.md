## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| id | String | Да |  |
| update_api_page_data | models::UpdateApiPageData | Да |  |

## Одговор

Враћа: [`PatchPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_page_api_response.rs)

## Пример

[inline-code-attrs-start title = 'patch_page пример'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn update_page() -> Result<PatchPageApiResponse, Error> {
    let params: PatchPageParams = PatchPageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        id: "news/product-update-2026".to_string(),
        update_api_page_data: models::UpdateApiPageData {
            title: Some("June 2026 Product Update".to_string()),
            slug: Some("news/product-update-2026".to_string()),
            description: Some("Summarizes June releases and roadmap changes".to_string()),
            is_published: Some(true),
            content: Some("<p>We shipped performance improvements and new integrations.</p>".to_string()),
        },
    };

    let response: PatchPageApiResponse = patch_page(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]

---