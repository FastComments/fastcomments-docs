---
Листa страница за tenant. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба.
Потребно је да `enableFChat` буде true у решеном прилагођеном конфигу за сваку страницу.
Странице које захтевају SSO се филтрирају у складу са приступним групама корисника који шаље захтев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор за пагинацију враћен као `nextCursor` из претходног захтева. Повезан са истим `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опционални филтер префикса наслова који није осетљив на велика/мала слова. |
| sortBy | string | query | No | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно). |
| hasComments | boolean | query | No | Ако је true, враћају се само странице са најмање једним коментаром. |

## Response

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Пример

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Непрозирни курсор за пагинацију враћен као `nextCursor` из претходног захтева. Повезан са истим `sortBy`.
  limit: 56, # Integer | 1..200, подразумевано 50
  q: 'q_example', # String | Опционални филтер префикса наслова који није осетљив на велика/мала слова.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (абецедно).
  has_comments: true # Boolean | Ако је true, враћају се само странице са најмање једним коментаром.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]

---