req
tenantId
urlId

## Parametre

| Name | Type | Location | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja |  |
| page | integer | query | Nej |  |
| direction | string | query | Nej |  |
| sso | string | query | Nej |  |
| skip | integer | query | Nej |  |
| skipChildren | integer | query | Nej |  |
| limit | integer | query | Nej |  |
| limitChildren | integer | query | Nej |  |
| countChildren | boolean | query | Nej |  |
| fetchPageForCommentId | string | query | Nej |  |
| includeConfig | boolean | query | Nej |  |
| countAll | boolean | query | Nej |  |
| includei10n | boolean | query | Nej |  |
| locale | string | query | Nej |  |
| modules | string | query | Nej |  |
| isCrawler | boolean | query | Nej |  |
| includeNotificationCount | boolean | query | Nej |  |
| asTree | boolean | query | Nej |  |
| maxTreeDepth | integer | query | Nej |  |
| useFullTranslationIds | boolean | query | Nej |  |
| parentId | string | query | Nej |  |
| searchText | string | query | Nej |  |
| hashTags | array | query | Nej |  |
| userId | string | query | Nej |  |
| customConfigStr | string | query | Nej |  |
| afterCommentId | string | query | Nej |  |
| beforeCommentId | string | query | Nej |  |

## Svar

Returnerer: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_public200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_comments_public Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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