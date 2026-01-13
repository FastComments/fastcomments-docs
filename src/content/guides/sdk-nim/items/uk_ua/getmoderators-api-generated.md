## Параметри

| Name | Type | Обов'язковий | Опис |
|------|------|--------------|------|
| tenantId | string | Так |  |
| skip | float64 | Ні |  |

## Відповідь

Повертає: [`Option[GetModerators_200_response]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators200response.nim)

## Приклад

[inline-code-attrs-start title = 'Приклад getModerators'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if response.isSome:
  let moderators = response.get()
  echo "Moderators fetched successfully"
  echo moderators
[inline-code-end]

---