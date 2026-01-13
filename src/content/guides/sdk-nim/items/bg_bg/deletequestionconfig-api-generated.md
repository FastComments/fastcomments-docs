## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Не |  |

## Отговор

Връща: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за deleteQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-456")
if response.isSome:
  let respVal = response.get()
  echo "Delete succeeded for tenant my-tenant-123"
else:
  echo "Delete returned no data (status: ", $httpResponse.status, ")"
[inline-code-end]

---