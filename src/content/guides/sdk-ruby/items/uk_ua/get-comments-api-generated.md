## Параметри

| Name | Type | Розташування | Обов'язкове | Опис |
|------|------|--------------|-------------|------|
| tenantId | string | параметр запиту | Так |  |
| page | integer | параметр запиту | Ні |  |
| limit | integer | параметр запиту | Ні |  |
| skip | integer | параметр запиту | Ні |  |
| asTree | boolean | параметр запиту | Ні |  |
| skipChildren | integer | параметр запиту | Ні |  |
| limitChildren | integer | параметр запиту | Ні |  |
| maxTreeDepth | integer | параметр запиту | Ні |  |
| urlId | string | параметр запиту | Ні |  |
| userId | string | параметр запиту | Ні |  |
| anonUserId | string | параметр запиту | Ні |  |
| contextUserId | string | параметр запиту | Ні |  |
| hashTag | string | параметр запиту | Ні |  |
| parentId | string | параметр запиту | Ні |  |
| direction | string | параметр запиту | Ні |  |

## Відповідь

Повертає: [`GetComments200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments200_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштувати авторизацію за API-ключем: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, наприклад 'Bearer' (за замовчуванням nil)
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