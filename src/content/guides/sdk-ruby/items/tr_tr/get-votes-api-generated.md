## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Evet |  |

## Yanıt

Döndürür: [`GetVotes200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_votes200_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_votes Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarı için bir önek ayarlamak üzere aşağıdaki satırın yorumunu kaldırın, örn. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 

begin
  
  result = api_instance.get_votes(tenant_id, url_id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_votes: #{e}"
end
[inline-code-end]

---