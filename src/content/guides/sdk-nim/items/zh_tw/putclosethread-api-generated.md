## 參數

| 名稱 | 類型 | 必填 | 描述 |
|------|------|----------|-------------|
| urlId | string | 是 |  |
| sso | string | 否 |  |

## 回應

回傳：[`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 範例

[inline-code-attrs-start title = 'putCloseThread 範例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putCloseThread(urlId = "news/article-title", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Thread closed successfully:", $apiResp
else:
  echo "Failed to close thread, HTTP response:", $httpResponse
[inline-code-end]