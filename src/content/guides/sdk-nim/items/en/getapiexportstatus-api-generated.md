## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | GetApiExportStatusOptions | No |  |

## Response

Returns: [`Option[ModerationExportStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_status_response.nim)

## Example

[inline-code-attrs-start title = 'getApiExportStatus Example'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (exportStatusOpt, httpResp) = client.getApiExportStatus(
  tenantId = "my-tenant-123",
  options = GetApiExportStatusOptions()
)
if exportStatusOpt.isSome:
  let exportStatus = exportStatusOpt.get()
  # use exportStatus as needed
[inline-code-end]
