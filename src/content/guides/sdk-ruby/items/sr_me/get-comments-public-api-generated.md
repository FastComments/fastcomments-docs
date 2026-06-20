req
tenantId
urlId

## Параметри

| Име | Тип | Локација | Захтевано | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | путања | Да |  |
| urlId | string | упит | Да |  |
| page | integer | упит | Не |  |
| direction | string | упит | Не |  |
| sso | string | упит | Не |  |
| skip | integer | упит | Не |  |
| skipChildren | integer | упит | Не |  |
| limit | integer | упит | Не |  |
| limitChildren | integer | упит | Не |  |
| countChildren | boolean | упит | Не |  |
| fetchPageForCommentId | string | упит | Не |  |
| includeConfig | boolean | упит | Не |  |
| countAll | boolean | упит | Не |  |
| includei10n | boolean | упит | Не |  |
| locale | string | упит | Не |  |
| modules | string | упит | Не |  |
| isCrawler | boolean | упит | Не |  |
| includeNotificationCount | boolean | упит | Не |  |
| asTree | boolean | упит | Не |  |
| maxTreeDepth | integer | упит | Не |  |
| useFullTranslationIds | boolean | упит | Не |  |
| parentId | string | упит | Не |  |
| searchText | string | упит | Не |  |
| hashTags | array | упит | Не |  |
| userId | string | упит | Не |  |
| customConfigStr | string | упит | Не |  |
| afterCommentId | string | упит | Не |  |
| beforeCommentId | string | упит | Не |  |

## Одговор

Враћа: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_response_with_presence_public_comment.rb)

## Пример

[inline-code-attrs-start title = 'get_comments_public Пример'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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