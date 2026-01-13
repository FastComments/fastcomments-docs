## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |
| updateQuestionConfigBody | UpdateQuestionConfigBody | Ne |  |

## Odgovor

Vrača: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer updateQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.updateQuestionConfig(
  tenantId = "my-tenant-123",
  id = "q-config-456",
  updateQuestionConfigBody = UpdateQuestionConfigBody()
)
if response.isSome:
  let updated = response.get()
  discard updated
  echo "Question config updated"
else:
  echo "Update did not return a result"
[inline-code-end]

---