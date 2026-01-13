## Parametreler

| Ad | Tür | Konum | Zorunlu | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |

## Yanıt

Dönüş Değeri: [`GetCachedNotificationCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_cached_notification_count200_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_cached_notification_count Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # Configure API key authorization: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Aşağıdaki satırın yorumunu kaldırarak API anahtarı için bir önek ayarlayın, örn. 'Bearer' (varsayılan: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_cached_notification_count(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_cached_notification_count: #{e}"
end
[inline-code-end]

---