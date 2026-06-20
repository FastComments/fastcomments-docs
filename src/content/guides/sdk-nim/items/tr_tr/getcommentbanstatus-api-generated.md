## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| commentId | string | Evet |  |
| sso | string | Hayır |  |

## Yanıt

Döndürür: [`Option[GetCommentBanStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_comment_ban_status_response.nim)

## Örnek

[inline-code-attrs-start title = 'getCommentBanStatus Örneği'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentBanStatus(commentId = "cmt-987654321", sso = "")

if response.isSome:
  let banStatus = response.get()
  echo "Ban status for comment cmt-987654321: ", banStatus
else:
  echo "No ban status returned for comment cmt-987654321"
[inline-code-end]

---