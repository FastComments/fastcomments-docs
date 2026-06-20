## פרמטרים

| שם | סוג | חובה | תיאור |
|------|------|----------|-------------|
| commentId | string | כן |  |
| direction | string | לא |  |
| sso | string | לא |  |

## תגובה

מחזיר: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postVote(commentId = "comment-4f3a9e", direction = "up", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlci0xMjMifQ.signedPart")
if response.isSome:
  let vote = response.get()
  echo "Vote recorded:", vote
else:
  echo "No vote returned"
[inline-code-end]

---