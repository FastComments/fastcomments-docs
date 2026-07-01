Upload and resize an image
============================

## Parameters

| Ime | Vrsta | Obvezno | Opis |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| file | std::path::PathBuf | Yes |  |
| size_preset | models::SizePreset | No |  |
| url_id | String | No |  |

## Odgovor

Vrne: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

## Primer

[inline-code-attrs-start title = 'upload_image Primer'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = UploadImageParams {
        tenant_id: "acme-corp-tenant".to_string(),
        file: std::path::PathBuf::from("/tmp/photo.jpg"),
        size_preset: Some(models::SizePreset::Medium),
        url_id: Some("news/article".to_string()),
    };
    let _response = upload_image(&configuration, params).await?;
    Ok(())
}
[inline-code-end]