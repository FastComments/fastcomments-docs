## פרמטרים

| שם | Type | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[ModerationAPIGetLogsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_moderation_api_get_logs_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getLogs'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getLogs(commentId = "cmt-8471f2d3", sso = "")
if response.isSome:
  let logs = response.get()
  echo "Fetched logs:", logs
[inline-code-end]

---