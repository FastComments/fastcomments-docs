req
tenantId
urlId

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| page | integer | query | Ні |  |
| direction | string | query | Ні |  |
| sso | string | query | Ні |  |
| skip | integer | query | Ні |  |
| skipChildren | integer | query | Ні |  |
| limit | integer | query | Ні |  |
| limitChildren | integer | query | Ні |  |
| countChildren | boolean | query | Ні |  |
| fetchPageForCommentId | string | query | Ні |  |
| includeConfig | boolean | query | Ні |  |
| countAll | boolean | query | Ні |  |
| includei10n | boolean | query | Ні |  |
| locale | string | query | Ні |  |
| modules | string | query | Ні |  |
| isCrawler | boolean | query | Ні |  |
| includeNotificationCount | boolean | query | Ні |  |
| asTree | boolean | query | Ні |  |
| maxTreeDepth | integer | query | Ні |  |
| useFullTranslationIds | boolean | query | Ні |  |
| parentId | string | query | Ні |  |
| searchText | string | query | Ні |  |
| hashTags | array | query | Ні |  |
| userId | string | query | Ні |  |
| customConfigStr | string | query | Ні |  |
| afterCommentId | string | query | Ні |  |
| beforeCommentId | string | query | Ні |  |

## Відповідь

Повертає: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_public200_response.rb)

## Приклад

[inline-code-attrs-start title = 'Приклад get_comments_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  page: 56, # Integer | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  sso: 'sso_example', # String | 
  skip: 56, # Integer | 
  skip_children: 56, # Integer | 
  limit: 56, # Integer | 
  limit_children: 56, # Integer | 
  count_children: true, # Boolean | 
  fetch_page_for_comment_id: 'fetch_page_for_comment_id_example', # String | 
  include_config: true, # Boolean | 
  count_all: true, # Boolean | 
  includei10n: true, # Boolean | 
  locale: 'locale_example', # String | 
  modules: 'modules_example', # String | 
  is_crawler: true, # Boolean | 
  include_notification_count: true, # Boolean | 
  as_tree: true, # Boolean | 
  max_tree_depth: 56, # Integer | 
  use_full_translation_ids: true, # Boolean | 
  parent_id: 'parent_id_example', # String | 
  search_text: 'search_text_example', # String | 
  hash_tags: ['inner_example'], # Array<String> | 
  user_id: 'user_id_example', # String | 
  custom_config_str: 'custom_config_str_example', # String | 
  after_comment_id: 'after_comment_id_example', # String | 
  before_comment_id: 'before_comment_id_example' # String | 
}

begin
  
  result = api_instance.get_comments_public(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_public: #{e}"
end
[inline-code-end]

---