## Παράμετροι

| Όνομα | Τύπος | Απαιτείται | Περιγραφή |
|------|------|----------|-------------|
| tenantId | string | Ναι |  |
| commentId | string | Ναι |  |
| editKey | string | Όχι |  |
| sso | string | Όχι |  |

## Απάντηση

Επιστρέφει: [`Option[PublicAPIGetCommentTextResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_public_api_get_comment_text_response.nim)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα getCommentText'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getCommentText(tenantId = "my-tenant-123", commentId = "cmt-987654321", editKey = "", sso = "")

if response.isSome:
  let commentTextResp = response.get()
  echo commentTextResp
else:
  echo "No comment text returned"
[inline-code-end]

---