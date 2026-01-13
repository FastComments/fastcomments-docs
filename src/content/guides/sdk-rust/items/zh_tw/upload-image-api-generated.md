上傳並調整影像大小

## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|------|
| tenant_id | String | 是 |  |
| file | std::path::PathBuf | 是 |  |
| size_preset | models::SizePreset | 否 |  |
| url_id | String | 否 |  |

## 回應

回傳：[`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---