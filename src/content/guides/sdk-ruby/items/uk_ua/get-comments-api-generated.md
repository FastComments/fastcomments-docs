## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| page | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| skip | integer | query | Ні |  |
| asTree | boolean | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| urlId | string | query | Ні |  |
| userId | string | query | Ні |  |
| anonUserId | string | query | Ні |  |
| contextUserId | string | query | Ні |  |
| hashTag | string | query | Ні |  |
| parentId | string | query | Ні |  |
| direction | string | query | Ні |  |
| fromDate | integer | query | Ні |  |
| toDate | integer | query | Ні |  |

## Відповідь

Повертає: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_comments_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# налаштування авторизації
FastCommentsClient.configure do |config|
  # Налаштування авторизації API-ключем: api_key
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
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  from_date: 789, # Integer | 
  to_date: 789 # Integer | 
}

begin
  
  result = api_instance.get_comments(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_comments: #{e}"
end
[inline-code-end]

---