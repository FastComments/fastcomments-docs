Список страниц для тенанта. Используется десктоп-клиентом FChat для заполнения списка комнат.
Требуется, чтобы `enableFChat` было true в итоговой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются по доступу групп запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Нет | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | Нет | 1..200, по умолчанию 50 |
| q | string | query | Нет | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | Нет | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту). |
| hasComments | boolean | query | Нет | Если true, возвращать только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Непрозрачный курсор постраничной навигации, возвращённый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`.
  limit: 56, # Integer | 1..200, по умолчанию 50
  q: 'q_example', # String | Необязательный регистронезависимый фильтр по префиксу заголовка.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала самые новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту).
  has_comments: true # Boolean | Если true, возвращать только страницы с хотя бы одним комментарием.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]