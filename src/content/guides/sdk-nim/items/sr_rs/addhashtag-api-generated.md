## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| createHashTagBody | CreateHashTagBody | Не |  |

## Одговор

Враћа: [`Option[CreateHashTagResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_create_hash_tag_response.nim)

## Пример

[inline-code-attrs-start title = 'Primer addHashTag'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (hashTagOpt, httpResp) = client.addHashTag(
  tenantId = "my-tenant-123",
  createHashTagBody = CreateHashTagBody(),
)

if hashTagOpt.isSome:
  let tag = hashTagOpt.get()
[inline-code-end]