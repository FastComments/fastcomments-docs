## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| id | string | path | Evet |  |
| sendEmail | string | query | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Örnek

[inline-code-attrs-start title = 'delete_moderator Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# Yetkilendirmeyi yapılandır
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Aşağıdaki satırın yorumunu kaldırarak API anahtarı için bir önek ayarlayın, ör. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  send_email: 'send_email_example' # String | 
}

begin
  
  result = api_instance.delete_moderator(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_moderator: #{e}"
end
[inline-code-end]

---