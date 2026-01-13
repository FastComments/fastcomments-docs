העלאה ושינוי גודל של תמונה

## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenant_id | String | כן |  |
| file | std::path::PathBuf | כן |  |
| size_preset | models::SizePreset | לא |  |
| url_id | String | לא |  |

## תגובה

מחזיר: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---