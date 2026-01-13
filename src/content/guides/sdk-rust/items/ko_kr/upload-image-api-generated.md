이미지 업로드 및 크기 조정

## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenant_id | String | 예 |  |
| file | std::path::PathBuf | 예 |  |
| size_preset | models::SizePreset | 아니오 |  |
| url_id | String | 아니오 |  |

## 응답

반환: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)