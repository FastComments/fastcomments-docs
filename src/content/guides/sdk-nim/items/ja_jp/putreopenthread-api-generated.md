## パラメータ

| 名前 | 型 | 必須 | 説明 |
|------|------|----------|-------------|
| urlId | string | はい |  |
| sso | string | いいえ |  |

## レスポンス

返却: [`Option[APIEmptyResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_api_empty_response.nim)

## 例

[inline-code-attrs-start title = 'putReopenThread の例'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.putReopenThread(urlId = "news/2026-election-analysis", sso = "")
if response.isSome:
  let apiResp = response.get()
  echo "Reopen succeeded, response: ", apiResp
else:
  echo "Reopen failed, HTTP status: ", httpResponse.status
[inline-code-end]

---