## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Ne |  |

## Odgovor

Vraća: [`Option[GetQuestionConfig_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_question_config200response.nim)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionConfig'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getQuestionConfig(tenantId = "my-tenant-123", id = "qcfg-98765")
if response.isSome:
  let config = response.get()
  echo "Received question config for tenant:", " my-tenant-123"
[inline-code-end]

---