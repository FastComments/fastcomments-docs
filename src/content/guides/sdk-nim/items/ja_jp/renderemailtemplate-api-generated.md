## Parameters

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | いいえ |  |
| locale | string = "" | いいえ |  |

## レスポンス

戻り値: [`Option[RenderEmailTemplateResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template_response.nim)

## 例

[inline-code-attrs-start title = 'renderEmailTemplate の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let body = RenderEmailTemplateBody()
let (responseOpt, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = body, locale = "en-US")
if responseOpt.isSome:
  let response = responseOpt.get()
  discard response
discard httpResponse
[inline-code-end]

---