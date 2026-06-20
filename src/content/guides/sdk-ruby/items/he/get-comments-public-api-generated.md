req
tenantId
urlId

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן |  |
| page | integer | query | לא |  |
| direction | string | query | לא |  |
| sso | string | query | לא |  |
| skip | integer | query | לא |  |
| skipChildren | integer | query | לא |  |
| limit | integer | query | לא |  |
| limitChildren | integer | query | לא |  |
| countChildren | boolean | query | לא |  |
| fetchPageForCommentId | string | query | לא |  |
| includeConfig | boolean | query | לא |  |
| countAll | boolean | query | לא |  |
| includei10n | boolean | query | לא |  |
| locale | string | query | לא |  |
| modules | string | query | לא |  |
| isCrawler | boolean | query | לא |  |
| includeNotificationCount | boolean | query | לא |  |
| asTree | boolean | query | לא |  |
| maxTreeDepth | integer | query | לא |  |
| useFullTranslationIds | boolean | query | לא |  |
| parentId | string | query | לא |  |
| searchText | string | query | לא |  |
| hashTags | array | query | לא |  |
| userId | string | query | לא |  |
| customConfigStr | string | query | לא |  |
| afterCommentId | string | query | לא |  |
| beforeCommentId | string | query | לא |  |

## תשובה

מחזיר: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_response_with_presence_public_comment.rb)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-get_comments_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # מחרוזת | 
url_id = 'url_id_example' # מחרוזת | 
opts = {
  page: 56, # מספר שלם | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  sso: 'sso_example', # מחרוזת | 
  skip: 56, # מספר שלם | 
  skip_children: 56, # מספר שלם | 
  limit: 56, # מספר שלם | 
  limit_children: 56, # מספר שלם | 
  count_children: true, # בוליאני | 
  fetch_page_for_comment_id: 'fetch_page_for_comment_id_example', # מחרוזת | 
  include_config: true, # בוליאני | 
  count_all: true, # בוליאני | 
  includei10n: true, # בוליאני | 
  locale: 'locale_example', # מחרוזת | 
  modules: 'modules_example', # מחרוזת | 
  is_crawler: true, # בוליאני | 
  include_notification_count: true, # בוליאני | 
  as_tree: true, # בוליאני | 
  max_tree_depth: 56, # מספר שלם | 
  use_full_translation_ids: true, # בוליאני | 
  parent_id: 'parent_id_example', # מחרוזת | 
  search_text: 'search_text_example', # מחרוזת | 
  hash_tags: ['inner_example'], # מערך<String> | 
  user_id: 'user_id_example', # מחרוזת | 
  custom_config_str: 'custom_config_str_example', # מחרוזת | 
  after_comment_id: 'after_comment_id_example', # מחרוזת | 
  before_comment_id: 'before_comment_id_example' # מחרוזת | 
}

begin
  
  result = api_instance.get_comments_public(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_public: #{e}"
end
[inline-code-end]

---