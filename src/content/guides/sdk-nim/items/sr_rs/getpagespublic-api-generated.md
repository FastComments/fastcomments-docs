Наводи странице за закупца (tenant). Користи FChat десктоп клијент да попуни листу својих соба.
Захтева да `enableFChat` буде true у резолвованој прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у односу на приступ групама тражеог корисника.

## Параметри

| Назив | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| options | GetPagesPublicOptions | Не |  |

## Одговор

Враћа: [`Option[GetPublicPagesResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_get_public_pages_response.nim)

## Пример

[inline-code-attrs-start title = 'Пример за getPagesPublic'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (response, httpResponse) = client.getPagesPublic(tenantId = "my-tenant-123", options = GetPagesPublicOptions())
if response.isSome:
  let pages = response.get()
  echo pages
[inline-code-end]