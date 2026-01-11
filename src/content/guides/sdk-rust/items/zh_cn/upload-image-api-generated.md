---
上传并调整图片大小

## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | 是 |  |
| file | std::path::PathBuf | 是 |  |
| size_preset | models::SizePreset | 否 |  |
| url_id | String | 否 |  |

## 响应

返回: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---