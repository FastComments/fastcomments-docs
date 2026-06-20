---
Зведена інформація про користувачів для орендаря. За наданими userIds повертає відображувану інформацію з User / SSOUser.
Використовується віджетом коментарів для доповнення користувачів, які щойно з'явилися через подію присутності.
Без контексту сторінки: конфіденційність застосовується однаково (приватні профілі приховані).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds, розділені комами. |

## Response

Повертає: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Example

[inline-code-attrs-start title = 'Приклад get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds, розділені комами.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]

---