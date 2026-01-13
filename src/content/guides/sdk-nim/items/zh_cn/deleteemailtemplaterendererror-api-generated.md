## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 否 |  |
| errorId | string | 否 |  |

## 响应

返回: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## 示例

[inline-code-attrs-start title = 'deleteEmailTemplateRenderError 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteEmailTemplateRenderError(
  tenantId = "my-tenant-123",
  id = "welcome-email-template",
  errorId = "render-error-2026"
)
if response.isSome:
  let flagResp = response.get()
  discard flagResp
[inline-code-end]

---