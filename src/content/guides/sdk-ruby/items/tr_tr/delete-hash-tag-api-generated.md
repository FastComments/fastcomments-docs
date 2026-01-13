## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tag | string | path | Evet |  |
| tenantId | string | query | Hayır |  |

## Yanıt

Döndürür: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/flag_comment_public200_response.rb)

## Örnek

[inline-code-attrs-start title = 'delete_hash_tag Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirmeyi ayarla
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarına bir önek ayarlamak için aşağıdaki satırın yorumunu kaldırın, örn. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tag = 'tag_example' # String | 
opts = {
  tenant_id: 'tenant_id_example', # String | 
  delete_hash_tag_request: FastCommentsClient::DeleteHashTagRequest.new # DeleteHashTagRequest | 
}

begin
  
  result = api_instance.delete_hash_tag(tag, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->delete_hash_tag: #{e}"
end
[inline-code-end]