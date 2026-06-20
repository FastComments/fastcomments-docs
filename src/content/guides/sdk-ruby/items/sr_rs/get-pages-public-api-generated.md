Списак страница за тенанта. Користи га FChat десктоп клијент за попуњавање своје листе соба.
Захтева да је `enableFChat` постављен на true у решеној прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају на основу групног приступа корисника који шаље захтев.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Нечитљив курсор пагинације враћен као `nextCursor` из претходног захтева. Повезан је са истим `sortBy`. |
| limit | integer | query | Не | 1..200, подразумевано 50 |
| q | string | query | Не | Опциони филтер префикса наслова који није осетљив на велика/мала слова. |
| sortBy | string | query | Не | Редослед сортирања. `updatedAt` (подразумевано, најновији први), `commentCount` (највише коментара прво), или `title` (алфабетски). |
| hasComments | boolean | query | Не | Ако је true, враћају се само странице које имају бар један коментар. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Пример

[inline-code-attrs-start title = 'get_pages_public пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Нечитљив курсор пагинације који је враћен као `nextCursor` из претходног захтева. Повезан је са истим `sortBy`.
  limit: 56, # Integer | 1..200, подразумевано 50
  q: 'q_example', # String | Опциони филтер префикса наслова који није осетљив на велика/мала слова.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновији први), `commentCount` (највише коментара прво), или `title` (алфабетски).
  has_comments: true # Boolean | Ако је true, враћају се само странице које имају бар један коментар.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]