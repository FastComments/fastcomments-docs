## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| skip | number | query | No |  |

## Yanıt

Döndürür: [`GetModeratorsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_moderators_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_moderators Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi yapılandır
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarı için bir önek ayarlamak üzere aşağıdaki satırın yorumunu kaldırın, ör. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_moderators(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_moderators: #{e}"
end
[inline-code-end]