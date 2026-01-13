## パラメータ

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| renderEmailTemplateBody | RenderEmailTemplateBody | いいえ |  |
| locale | string | いいえ |  |

## レスポンス

戻り値: [`Option[RenderEmailTemplate_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_render_email_template200response.nim)

## 例

[inline-code-attrs-start title = 'renderEmailTemplate の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let renderBody = RenderEmailTemplateBody(templateId: "comment-notification", subject: "New comment on your article", variables: @["John Doe", "news/global-climate"])
let (response, httpResponse) = client.renderEmailTemplate(tenantId = "my-tenant-123", renderEmailTemplateBody = renderBody, locale = "en-US")
if response.isSome:
  let rendered = response.get()
  echo rendered
[inline-code-end]

---