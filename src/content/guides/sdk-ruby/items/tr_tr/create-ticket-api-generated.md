## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | sorgu | Evet |  |
| userId | string | sorgu | Evet |  |

## Yanıt

Döndürür: [`CreateTicketResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_ticket_response.rb)

## Örnek

[inline-code-attrs-start title = 'create_ticket Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandırın: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Aşağıdaki satırın yorumunu kaldırarak API anahtarına bir önek ayarlayın, örn. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
user_id = 'user_id_example' # String | 
create_ticket_body = FastCommentsClient::CreateTicketBody.new({subject: 'subject_example'}) # CreateTicketBody | 

begin
  
  result = api_instance.create_ticket(tenant_id, user_id, create_ticket_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_ticket: #{e}"
end
[inline-code-end]