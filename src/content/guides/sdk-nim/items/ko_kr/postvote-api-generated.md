## 매개변수

| Name | Type | Required | Description |
|------|------|----------|-------------|
| commentId | string | 예 |  |
| direction | string | 아니요 |  |
| sso | string | 아니요 |  |

## 응답

반환: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## 예제

[inline-code-attrs-start title = 'postVote 예제'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postVote(commentId = "comment-4f3a9e", direction = "up", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlci0xMjMifQ.signedPart")
if response.isSome:
  let vote = response.get()
  echo "Vote recorded:", vote
else:
  echo "No vote returned"
[inline-code-end]

---