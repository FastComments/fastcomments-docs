## Paramètres

| Nom | Type | Obligatoire | Description |
|------|------|----------|-------------|
| tenantId | string | Oui |  |
| options | GetApiExportStatusOptions | Non |  |

## Réponse

Renvoie : [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Exemple

[inline-code-attrs-start title = 'Exemple getApiExportStatus'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (exportStatusOpt, httpResp) = client.getApiExportStatus(
  tenantId = "my-tenant-123",
  options = GetApiExportStatusOptions()
)
if exportStatusOpt.isSome:
  let exportStatus = exportStatusOpt.get()
  # utiliser exportStatus selon les besoins
[inline-code-end]