req
tenantId
urlId

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui |  |
| page | integer | query | Non |  |
| direction | string | query | Non |  |
| sso | string | query | Non |  |
| skip | integer | query | Non |  |
| skipChildren | integer | query | Non |  |
| limit | integer | query | Non |  |
| limitChildren | integer | query | Non |  |
| countChildren | boolean | query | Non |  |
| fetchPageForCommentId | string | query | Non |  |
| includeConfig | boolean | query | Non |  |
| countAll | boolean | query | Non |  |
| includei10n | boolean | query | Non |  |
| locale | string | query | Non |  |
| modules | string | query | Non |  |
| isCrawler | boolean | query | Non |  |
| includeNotificationCount | boolean | query | Non |  |
| asTree | boolean | query | Non |  |
| maxTreeDepth | integer | query | Non |  |
| useFullTranslationIds | boolean | query | Non |  |
| parentId | string | query | Non |  |
| searchText | string | query | Non |  |
| hashTags | array | query | Non |  |
| userId | string | query | Non |  |
| customConfigStr | string | query | Non |  |
| afterCommentId | string | query | Non |  |
| beforeCommentId | string | query | Non |  |

## Réponse

Retourne: [`GetCommentsPublic200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_public200_response.rb)

## Exemple

[inline-code-attrs-start title = 'Exemple de get_comments_public'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Chaîne | 
url_id = 'url_id_example' # Chaîne | 
opts = {
  page: 56, # Entier | 
  direction: FastCommentsClient::SortDirections::OF, # SortDirections | 
  sso: 'sso_example', # Chaîne | 
  skip: 56, # Entier | 
  skip_children: 56, # Entier | 
  limit: 56, # Entier | 
  limit_children: 56, # Entier | 
  count_children: true, # Booléen | 
  fetch_page_for_comment_id: 'fetch_page_for_comment_id_example', # Chaîne | 
  include_config: true, # Booléen | 
  count_all: true, # Booléen | 
  includei10n: true, # Booléen | 
  locale: 'locale_example', # Chaîne | 
  modules: 'modules_example', # Chaîne | 
  is_crawler: true, # Booléen | 
  include_notification_count: true, # Booléen | 
  as_tree: true, # Booléen | 
  max_tree_depth: 56, # Entier | 
  use_full_translation_ids: true, # Booléen | 
  parent_id: 'parent_id_example', # Chaîne | 
  search_text: 'search_text_example', # Chaîne | 
  hash_tags: ['inner_example'], # Tableau<String> | 
  user_id: 'user_id_example', # Chaîne | 
  custom_config_str: 'custom_config_str_example', # Chaîne | 
  after_comment_id: 'after_comment_id_example', # Chaîne | 
  before_comment_id: 'before_comment_id_example' # Chaîne | 
}

begin
  
  result = api_instance.get_comments_public(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_comments_public: #{e}"
end
[inline-code-end]