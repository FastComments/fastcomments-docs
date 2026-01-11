---
Upload og ændring af størrelse på et billede

## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenant_id | String | Ja |  |
| file | std::path::PathBuf | Ja |  |
| size_preset | models::SizePreset | Nej |  |
| url_id | String | Nej |  |

## Svar

Returnerer: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---