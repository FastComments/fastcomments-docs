## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | はい |  |
| commentId | string | はい |  |
| broadcastId | string | いいえ |  |
| sso | string | いいえ |  |

## Response

戻り値: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'lockComment の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.lockComment(tenantId = "news-tenant-42", commentId = "cmt-8f3a2b9d", broadcastId = "", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Locked comment successfully for tenant news-tenant-42"
else:
  echo "Failed to lock comment, HTTP status: ", $httpResponse.status
[inline-code-end]