---
## Параметри

| Ім'я | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Ні |  |

## Відповідь

Повертає: [`Option[FlagCommentPublic_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_flag_comment_public200response.nim)

## Приклад

[inline-code-attrs-start title = 'deleteQuestionConfig Приклад'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.deleteQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-456")
if response.isSome:
  let respVal = response.get()
  echo "Delete succeeded for tenant my-tenant-123"
else:
  echo "Delete returned no data (status: ", $httpResponse.status, ")"
[inline-code-end]

---