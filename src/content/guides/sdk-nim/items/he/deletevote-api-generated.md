## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | לא |  |
| editKey | string | לא |  |

## תגובה

מחזיר: [`Option[VoteDeleteResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_vote_delete_response.nim)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-deleteVote'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteVote(tenantId = "my-tenant-123", id = "vote-7f3b2a", editKey = "")
if response.isSome:
  let voteDelete = response.get()
  echo "Vote deleted successfully"
else:
  echo "Failed to delete vote"
[inline-code-end]

---