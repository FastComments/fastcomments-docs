छवि अपलोड और रिसाइज़ करें

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| file | std::path::PathBuf | हाँ |  |
| size_preset | models::SizePreset | नहीं |  |
| url_id | String | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'upload_image उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
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