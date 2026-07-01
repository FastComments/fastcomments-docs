## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|--------------|-----------|
| tenantId | string | Sim |  |
| options | GetApiExportStatusOptions | Não |  |

## Resposta

Retorna: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getApiExportStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (exportStatusOpt, httpResp) = client.getApiExportStatus(
  tenantId = "my-tenant-123",
  options = GetApiExportStatusOptions()
)
if exportStatusOpt.isSome:
  let exportStatus = exportStatusOpt.get()
  # usar exportStatus conforme necessário
[inline-code-end]