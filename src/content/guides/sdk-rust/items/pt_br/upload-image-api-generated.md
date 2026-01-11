---
Enviar e redimensionar uma imagem

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|------------|-----------|
| tenant_id | String | Sim |  |
| file | std::path::PathBuf | Sim |  |
| size_preset | models::SizePreset | Não |  |
| url_id | String | Não |  |

## Resposta

Retorna: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/upload_image_response.rs)

---