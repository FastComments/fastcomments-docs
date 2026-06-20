Изброява страници за наемател. Използва се от десктоп клиента FChat за попълване на своя списък със стаи.
Изисква `enableFChat` да бъде `true` в разрешената персонализирана конфигурация за всяка страница.
Страниците, които изискват SSO, се филтрират въз основа на груповия достъп на потребителя, който прави заявката.

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| cursor | string | query | Не | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`. |
| limit | integer | query | Не | 1..200, по подразбиране 50 |
| q | string | query | Не | Незадължителен регистронезависим филтър по префикс на заглавието. |
| sortBy | string | query | Не | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (азбучно). |
| hasComments | boolean | query | Не | Ако е true, връща само страници с поне един коментар. |

## Отговор

Връща: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Пример

[inline-code-attrs-start title = 'get_pages_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Непрозрачен курсор за пагинация, върнат като `nextCursor` от предишна заявка. Свързан със същия `sortBy`.
  limit: 56, # Integer | 1..200, по подразбиране 50
  q: 'q_example', # String | Незадължителен регистронезависим филтър по префикс на заглавието.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Ред на сортиране. `updatedAt` (по подразбиране, най-новите първи), `commentCount` (най-много коментари първи) или `title` (азбучно).
  has_comments: true # Boolean | Ако е true, връща само страници с поне един коментар.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]