## Параметри

| Име | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| skip | float64 | No |  |

## Одговор

Враћа: [`Option[GetModeratorsResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_moderators_response.nim)

## Пример

[inline-code-attrs-start title = 'getModerators Пример'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (moderatorsOpt, httpResp) = client.getModerators(tenantId = "my-tenant-123", skip = 0.0)
if moderatorsOpt.isSome:
  let moderators = moderatorsOpt.get()
  echo moderators
[inline-code-end]