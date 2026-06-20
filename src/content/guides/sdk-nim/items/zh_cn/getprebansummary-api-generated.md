## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|------|-------------|
| commentId | string | 是 |  |
| includeByUserIdAndEmail | bool | 否 |  |
| includeByIP | bool | 否 |  |
| includeByEmailDomain | bool | 否 |  |
| sso | string | 否 |  |

## 响应

返回： [`Option[PreBanSummary]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_pre_ban_summary.nim)

## 示例

[inline-code-attrs-start title = 'getPreBanSummary 示例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let commentId = "cmt-7423"
let (response, httpResponse) = client.getPreBanSummary(
  commentId = commentId,
  includeByUserIdAndEmail = false,
  includeByIP = false,
  includeByEmailDomain = false,
  sso = ""
)
if response.isSome:
  let preBanSummary = response.get()
  discard preBanSummary
else:
  discard httpResponse
[inline-code-end]

---