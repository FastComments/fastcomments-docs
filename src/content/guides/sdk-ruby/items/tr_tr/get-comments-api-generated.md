## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| page | integer | query | Hayır |  |
| limit | integer | query | Hayır |  |
| skip | integer | query | Hayır |  |
| asTree | boolean | query | Hayır |  |
| skipChildren | integer | query | Hayır |  |
| limitChildren | integer | query | Hayır |  |
| maxTreeDepth | integer | query | Hayır |  |
| urlId | string | query | Hayır |  |
| userId | string | query | Hayır |  |
| anonUserId | string | query | Hayır |  |
| contextUserId | string | query | Hayır |  |
| hashTag | string | query | Hayır |  |
| parentId | string | query | Hayır |  |
| direction | string | query | Hayır |  |

## Yanıt

Döndürür: [`GetComments200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments200_response.rb)

## Örnek

[inline-code-attrs-start title = 'get_comments Örneği'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# yetkilendirme kurulumu
FastCommentsClient.configure do |config|
  # API anahtarı yetkilendirmesini yapılandır: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # API anahtarı için bir önek ayarlamak üzere aşağıdaki satırın yorumunu kaldırın, örn. 'Bearer' (varsayılan nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  page: 56, # Integer | 
  limit: 56, # Integer | 
  skip: 56, # Integer | 
  as_tree: true, # Boolean | 
  skip_children: 56, # Integer | 
  limit_children: 56, # Integer | 
  max_tree_depth: 56, # Integer | 
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  anon_user_id: 'anon_user_id_example', # String | 
  context_user_id: 'context_user_id_example', # String | 
  hash_tag: 'hash_tag_example', # String | 
  parent_id: 'parent_id_example', # String | 
  direction: FastCommentsClient::SortDirections::OF # SortDirections | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]

---