## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| page | integer | query | לא |  |
| limit | integer | query | לא |  |
| skip | integer | query | לא |  |
| asTree | boolean | query | לא |  |
| skipChildren | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| urlId | string | query | לא |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |
| contextUserId | string | query | לא |  |
| hashTag | string | query | לא |  |
| parentId | string | query | לא |  |
| direction | string | query | לא |  |
| fromDate | integer | query | לא |  |
| toDate | integer | query | לא |  |

## תגובה

מחזיר: [`APIGetCommentsResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_comments_response.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# הגדרת הרשאה
FastCommentsClient.configure do |config|
  # קונפיגורציה של מפתח ה-API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # הסר את ההערה מהשורה הבאה כדי להגדיר קידומת למפתח ה-API, לדוגמה 'Bearer' (ברירת מחדל: nil)
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