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
