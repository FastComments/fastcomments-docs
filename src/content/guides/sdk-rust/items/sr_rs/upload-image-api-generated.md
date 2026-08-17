Upload and resize an image

## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenant_id | String | Да |  |
| file | std::path::PathBuf | Да |  |
| size_preset | models::SizePreset | Не |  |
| url_id | String | Не |  |

## Одговор

Враћа: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

## Пример

[inline-code-attrs-start title = 'Пример upload_image'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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