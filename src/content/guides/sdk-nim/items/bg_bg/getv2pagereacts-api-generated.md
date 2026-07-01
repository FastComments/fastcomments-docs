## Параметри

| Име | Тип | Задължително | Описание |
|------|------|--------------|----------|
| tenantId | string | Да |  |
| urlId | string | Да |  |

## Отговор

Връща: [`Option[GetV2PageReacts]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_v2_page_reacts.nim)

## Пример

[inline-code-attrs-start title = 'getV2PageReacts Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (reactsOpt, httpResp) = client.getV2PageReacts(tenantId = "my-tenant-123", urlId = "news/article-title")
if reactsOpt.isSome:
  let reacts = reactsOpt.get()
  echo reacts
[inline-code-end]

---