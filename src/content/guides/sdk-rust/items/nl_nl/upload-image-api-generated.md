Een afbeelding uploaden en schalen

## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| file | std::path::PathBuf | Ja |  |
| size_preset | models::SizePreset | Nee |  |
| url_id | String | Nee |  |

## Respons

Retourneert: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)