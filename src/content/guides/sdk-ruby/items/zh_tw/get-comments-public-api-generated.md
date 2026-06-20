req
tenantId
urlId

## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| urlId | string | query | 是 |  |
| page | integer | query | 否 |  |
| direction | string | query | 否 |  |
| sso | string | query | 否 |  |
| skip | integer | query | 否 |  |
| skipChildren | integer | query | 否 |  |
| limit | integer | query | 否 |  |
| limitChildren | integer | query | 否 |  |
| countChildren | boolean | query | 否 |  |
| fetchPageForCommentId | string | query | 否 |  |
| includeConfig | boolean | query | 否 |  |
| countAll | boolean | query | 否 |  |
| includei10n | boolean | query | 否 |  |
| locale | string | query | 否 |  |
| modules | string | query | 否 |  |
| isCrawler | boolean | query | 否 |  |
| includeNotificationCount | boolean | query | 否 |  |
| asTree | boolean | query | 否 |  |
| maxTreeDepth | integer | query | 否 |  |
| useFullTranslationIds | boolean | query | 否 |  |
| parentId | string | query | 否 |  |
| searchText | string | query | 否 |  |
| hashTags | array | query | 否 |  |
| userId | string | query | 否 |  |
| customConfigStr | string | query | 否 |  |
| afterCommentId | string | query | 否 |  |
| beforeCommentId | string | query | 否 |  |

## 回應

回傳: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_response_with_presence_public_comment.rb)

## 範例

[inline-code-attrs-start title = 'get_comments_public 範例'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # 字串 | 
url_id = 'url_id_example' # 字串 | 
opts = {
  page: 56, # 整數 | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  sso: 'sso_example', # 字串 | 
  skip: 56, # 整數 | 
  skip_children: 56, # 整數 | 
  limit: 56, # 整數 | 
  limit_children: 56, # 整數 | 
  count_children: true, # 布林值 | 
  fetch_page_for_comment_id: 'fetch_page_for_comment_id_example', # 字串 | 
  include_config: true, # 布林值 | 
  count_all: true, # 布林值 | 
  includei10n: true, # 布林值 | 
  locale: 'locale_example', # 字串 | 
  modules: 'modules_example', # 字串 | 
  is_crawler: true, # 布林值 | 
  include_notification_count: true, # 布林值 | 
  as_tree: true, # 布林值 | 
  max_tree_depth: 56, # 整數 | 
  use_full_translation_ids: true, # 布林值 | 
  parent_id: 'parent_id_example', # 字串 | 
  search_text: 'search_text_example', # 字串 | 
  hash_tags: ['inner_example'], # 字串陣列 | 
  user_id: 'user_id_example', # 字串 | 
  custom_config_str: 'custom_config_str_example', # 字串 | 
  after_comment_id: 'after_comment_id_example', # 字串 | 
  before_comment_id: 'before_comment_id_example' # 字串 | 
}

begin
  
  result = api_instance.get_comments_public(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_public: #{e}"
end
[inline-code-end]