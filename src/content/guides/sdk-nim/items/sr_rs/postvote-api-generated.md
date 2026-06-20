## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| commentId | string | Да |  |
| direction | string | Не |  |
| sso | string | Не |  |

## Одговор

Враћа: [`Option[VoteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример postVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.postVote(commentId = "comment-4f3a9e", direction = "up", sso = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjoidXNlci0xMjMifQ.signedPart")
if response.isSome:
  let vote = response.get()
  echo "Vote recorded:", vote
else:
  echo "No vote returned"
[inline-code-end]