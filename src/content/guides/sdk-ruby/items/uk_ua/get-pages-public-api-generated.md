Повертає список сторінок для тенанта. Використовується десктоп-клієнтом FChat для заповнення списку кімнат.
Потребує, щоб `enableFChat` був true у підсумковому custom config для кожної сторінки.
Сторінки, які вимагають SSO, відфільтровуються відповідно до групового доступу користувача, що робить запит.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| cursor | string | query | Ні | Непрозорий курсор пагінації, що повертається як `nextCursor` з попереднього запиту. Пов'язаний з тим же `sortBy`. |
| limit | integer | query | Ні | 1..200, за замовчуванням 50 |
| q | string | query | Ні | Необов'язковий нечутливий до регістру фільтр префіксу заголовка. |
| sortBy | string | query | Ні | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку сторінки з більшою кількістю коментарів) або `title` (за алфавітом). |
| hasComments | boolean | query | Ні | Якщо true, повертати лише сторінки з принаймні одним коментарем. |

## Відповідь

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_public_pages_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_pages_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  cursor: 'cursor_example', # String | Непрозорий курсор пагінації, що повертається як `nextCursor` з попереднього запиту. Пов'язаний з тим же `sortBy`.
  limit: 56, # Integer | 1..200, за замовчуванням 50
  q: 'q_example', # String | Необов'язковий нечутливий до регістру фільтр префіксу заголовка.
  sort_by: FastCommentsClient::PagesSortBy::UPDATED_AT, # PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку новіші), `commentCount` (спочатку сторінки з більшою кількістю коментарів), або `title` (за алфавітом).
  has_comments: true # Boolean | Якщо true, повертати лише сторінки з принаймні одним коментарем.
}

begin
  
  result = api_instance.get_pages_public(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_pages_public: #{e}"
end
[inline-code-end]