## 參數

| 名稱 | 類型 | 必填 | 說明 |
|------|------|------|-----|
| tenantId | string | 是 |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | 否 |  |
| locale | string = "" | 否 |  |

## 回應

返回：[`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## 範例

[inline-code-attrs-start title = 'renderEmailTemplate 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = RenderEmailTemplateBody()
let (responseOpt, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = body, locale = "en-US")
if responseOpt.isSome:
  let response = responseOpt.get()
  discard response
discard httpResponse
[inline-code-end]