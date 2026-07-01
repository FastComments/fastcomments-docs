## 参数

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| options | PostApiExportOptions | No |  |

## 响应

返回: [`Option[ModerationExportResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_export_response.nim)

## 示例

[inline-code-attrs-start title = 'postApiExport 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optExport, httpResp) = client.postApiExport(tenantId = "my-tenant-123", options = PostApiExportOptions())
if optExport.isSome:
  let export = optExport.get()
  echo export
[inline-code-end]