Масова информация за потребители за един наемател. За дадени userIds връща информация за показване от User / SSOUser.
Използва се от коментарния widget за обогатяване на потребители, които току-що се появиха чрез presence събитие.
Няма контекст на страница: поверителността се прилага по единен начин (частните профили се маскират).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | userIds, разделени със запетая. |

## Отговор

Връща: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_info_response.rb)

## Пример

[inline-code-attrs-start title = 'Пример за get_users_info'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
ids = 'ids_example' # String | userIds, разделени със запетая.

begin
  
  result = api_instance.get_users_info(tenant_id, ids)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_users_info: #{e}"
end
[inline-code-end]