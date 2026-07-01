## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| tag | string | path | Yes |  |

## Yanıt

Döndürür: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_empty_response.rb)

## Örnek

[inline-code-attrs-start title = 'delete_hash_tag Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarı için bir önek ayarlamak için aşağıdaki satırın yorumunu kaldırın, ör. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
tag = 'tag_example' # String | 
delete_hash_tag_request_body = FastCommentsClient::DeleteHashTagRequestBody.new # DeleteHashTagRequestBody | 

begin
  
  result = api_instance.delete_hash_tag(tenant_id, tag, delete_hash_tag_request_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "DefaultApi->delete_hash_tag çağrılırken hata: #{e}"
end
[inline-code-end]