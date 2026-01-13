Upload and resize an image

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| file | std::path::PathBuf | Yes |  |
| size_preset | models::SizePreset | No |  |
| url_id | String | No |  |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

## Example

[inline-code-attrs-start title = 'upload_image Example'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn run_upload() -> Result<UploadImageResponse, Error> {
    let params: UploadImageParams = UploadImageParams {
        tenant_id: String::from("acme-corp-tenant"),
        file: std::path::PathBuf::from("assets/images/news-article-hero.jpg"),
        size_preset: None,
        url_id: Some(String::from("news/article/2026/hero-image")),
    };
    let response: UploadImageResponse = upload_image(&configuration, params).await?;
    Ok(response)
}
[inline-code-end]
