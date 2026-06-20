---
Список страниц для тенанта. Используется десктопным клиентом FChat для заполнения списка комнат.
Требуется, чтобы в разрешённой пользовательской конфигурации для каждой страницы значение `enableFChat` было `true`.
Страницы, требующие SSO, фильтруются в соответствии с групповым доступом запрашивающего пользователя.

## Параметры

| Name | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке). |
| hasComments | boolean | query | No | Если `true`, возвращать только страницы, содержащие хотя бы один комментарий. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Пример

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`.
  limit: 56, # Integer | 1..200, по умолчанию 50
  q: 'q_example', # String | Необязательный регистронезависимый фильтр по префиксу заголовка.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (в алфавитном порядке).
  has_comments: true # Boolean | Если `true`, возвращать только страницы, содержащие хотя бы один комментарий.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---