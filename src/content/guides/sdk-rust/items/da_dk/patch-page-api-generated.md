## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| id | String | Yes |  |
| update_api_page_data | models::UpdateApiPageData | Yes |  |

## Svar

Returnerer: [`PatchPageApiResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/patch_page_api_response.rs)

## Eksempel

[inline-code-attrs-start title = 'patch_page Eksempel'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_patch_page() -> Result<(), Error> {
    let update = models::UpdateApiPageData {
        title: Some("Breaking News".into()),
        content: Some("Updated article content".into()),
        ..Default::default()
    };
    let params = PatchPageParams {
        tenant_id: "acme-corp-tenant".into(),
        id: "news/article".into(),
        update_api_page_data: update,
    };
    let _resp: PatchPageApiResponse = patch_page(&configuration, params).await?;
    Ok(())
}
[inline-code-end]