필수
tenantId
urlId

## 매개변수

| 이름 | 유형 | 위치 | 필수 | 설명 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 예 |  |
| urlId | string | query | 예 |  |
| page | integer | query | 아니요 |  |
| direction | string | query | 아니요 |  |
| sso | string | query | 아니요 |  |
| skip | integer | query | 아니요 |  |
| skipChildren | integer | query | 아니요 |  |
| limit | integer | query | 아니요 |  |
| limitChildren | integer | query | 아니요 |  |
| countChildren | boolean | query | 아니요 |  |
| fetchPageForCommentId | string | query | 아니요 |  |
| includeConfig | boolean | query | 아니요 |  |
| countAll | boolean | query | 아니요 |  |
| includei10n | boolean | query | 아니요 |  |
| locale | string | query | 아니요 |  |
| modules | string | query | 아니요 |  |
| isCrawler | boolean | query | 아니요 |  |
| includeNotificationCount | boolean | query | 아니요 |  |
| asTree | boolean | query | 아니요 |  |
| maxTreeDepth | integer | query | 아니요 |  |
| useFullTranslationIds | boolean | query | 아니요 |  |
| parentId | string | query | 아니요 |  |
| searchText | string | query | 아니요 |  |
| hashTags | array | query | 아니요 |  |
| userId | string | query | 아니요 |  |
| customConfigStr | string | query | 아니요 |  |
| afterCommentId | string | query | 아니요 |  |
| beforeCommentId | string | query | 아니요 |  |

## 응답

반환: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_public200_response.rb)

## 예제

[inline-code-attrs-start title = 'get_comments_public 예제'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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