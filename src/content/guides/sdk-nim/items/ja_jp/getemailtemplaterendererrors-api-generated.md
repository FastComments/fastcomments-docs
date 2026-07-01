## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | No |  |
| skip | float64 | No |  |

## レスポンス

返り値: [`Option[GetEmailTemplateRenderErrorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_email_template_render_errors_response.nim)

## 例

[inline-code-attrs-start title = 'getEmailTemplateRenderErrors 例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.getEmailTemplateRenderErrors(
  tenantId = "my-tenant-123",
  id = "welcome-template",
  skip = 0.0
)

if optResp.isSome:
  let resp = optResp.get()
  # 必要に応じて resp を使用
else:
  # 欠落した応答を処理
  discard
[inline-code-end]