画像をアップロードしてリサイズする

## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenant_id | String | はい |  |
| file | std::path::PathBuf | はい |  |
| size_preset | models::SizePreset | いいえ |  |
| url_id | String | いいえ |  |

## レスポンス

戻り値: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)