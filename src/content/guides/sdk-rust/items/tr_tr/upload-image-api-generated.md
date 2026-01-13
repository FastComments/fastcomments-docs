Bir resmi yükle ve yeniden boyutlandır

## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenant_id | String | Evet |  |
| file | std::path::PathBuf | Evet |  |
| size_preset | models::SizePreset | Hayır |  |
| url_id | String | Hayır |  |

## Yanıt

Döndürür: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---