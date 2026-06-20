req
tenantId
urlId

## Paramètres

| Name | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|------------|-------------|
| tenantId | string | chemin | Oui |  |
| urlId | string | requête | Oui |  |
| page | integer | requête | Non |  |
| direction | string | requête | Non |  |
| sso | string | requête | Non |  |
| skip | integer | requête | Non |  |
| skipChildren | integer | requête | Non |  |
| limit | integer | requête | Non |  |
| limitChildren | integer | requête | Non |  |
| countChildren | boolean | requête | Non |  |
| fetchPageForCommentId | string | requête | Non |  |
| includeConfig | boolean | requête | Non |  |
| countAll | boolean | requête | Non |  |
| includei10n | boolean | requête | Non |  |
| locale | string | requête | Non |  |
| modules | string | requête | Non |  |
| isCrawler | boolean | requête | Non |  |
| includeNotificationCount | boolean | requête | Non |  |
| asTree | boolean | requête | Non |  |
| maxTreeDepth | integer | requête | Non |  |
| useFullTranslationIds | boolean | requête | Non |  |
| parentId | string | requête | Non |  |
| searchText | string | requête | Non |  |
| hashTags | array | requête | Non |  |
| userId | string | requête | Non |  |
| customConfigStr | string | requête | Non |  |
| afterCommentId | string | requête | Non |  |
| beforeCommentId | string | requête | Non |  |

## Réponse

Retourne: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_response_with_presence_public_comment.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple get_comments_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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