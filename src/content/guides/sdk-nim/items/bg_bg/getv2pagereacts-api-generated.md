## Параметри

| Име | Тип | Задължително | Описание |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Отговор

Връща: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getV2PageReacts'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/elections/2026/us-primary-results")
if response.isSome:
  let reacts = response.get()
  echo "Received reacts: ", $reacts
else:
  echo "No reacts available"
[inline-code-end]

---