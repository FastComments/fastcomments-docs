必須
tenantId
urlId

## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| page | integer | query | いいえ |  |
| direction | string | query | いいえ |  |
| sso | string | query | いいえ |  |
| skip | integer | query | いいえ |  |
| skipChildren | integer | query | いいえ |  |
| limit | integer | query | いいえ |  |
| limitChildren | integer | query | いいえ |  |
| countChildren | boolean | query | いいえ |  |
| fetchPageForCommentId | string | query | いいえ |  |
| includeConfig | boolean | query | いいえ |  |
| countAll | boolean | query | いいえ |  |
| includei10n | boolean | query | いいえ |  |
| locale | string | query | いいえ |  |
| modules | string | query | いいえ |  |
| isCrawler | boolean | query | いいえ |  |
| includeNotificationCount | boolean | query | いいえ |  |
| asTree | boolean | query | いいえ |  |
| maxTreeDepth | integer | query | いいえ |  |
| useFullTranslationIds | boolean | query | いいえ |  |
| parentId | string | query | いいえ |  |
| searchText | string | query | いいえ |  |
| hashTags | array | query | いいえ |  |
| userId | string | query | いいえ |  |
| customConfigStr | string | query | いいえ |  |
| afterCommentId | string | query | いいえ |  |
| beforeCommentId | string | query | いいえ |  |

## レスポンス

戻り値: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_public200_response.rb)

## 例

[inline-code-attrs-start title = 'get_comments_public の例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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