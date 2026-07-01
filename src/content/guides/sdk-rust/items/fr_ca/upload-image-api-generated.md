Téléverser et redimensionner une image

## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenant_id | String | Oui |  |
| file | std::path::PathBuf | Oui |  |
| size_preset | models::SizePreset | Non |  |
| url_id | String | Non |  |

## Réponse

Renvoie : [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

## Exemple

[inline-code-attrs-start title = 'upload_image Exemple'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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