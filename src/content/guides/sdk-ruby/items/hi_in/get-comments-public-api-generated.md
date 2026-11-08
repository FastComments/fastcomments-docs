req
tenantId
urlId

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| page | integer | query | नहीं |  |
| direction | string | query | नहीं |  |
| sso | string | query | नहीं |  |
| skip | integer | query | नहीं |  |
| skipChildren | integer | query | नहीं |  |
| limit | integer | query | नहीं |  |
| limitChildren | integer | query | नहीं |  |
| countChildren | boolean | query | नहीं |  |
| fetchPageForCommentId | string | query | नहीं |  |
| includeConfig | boolean | query | नहीं |  |
| countAll | boolean | query | नहीं |  |
| includei10n | boolean | query | नहीं |  |
| locale | string | query | नहीं |  |
| modules | string | query | नहीं |  |
| isCrawler | boolean | query | नहीं |  |
| includeNotificationCount | boolean | query | नहीं |  |
| asTree | boolean | query | नहीं |  |
| maxTreeDepth | integer | query | नहीं |  |
| useFullTranslationIds | boolean | query | नहीं |  |
| parentId | string | query | नहीं |  |
| searchText | string | query | नहीं |  |
| hashTags | array | query | नहीं |  |
| userId | string | query | नहीं |  |
| customConfigStr | string | query | नहीं |  |
| afterCommentId | string | query | नहीं |  |
| beforeCommentId | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetCommentsResponseWithPresencePublicComment`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_comments_response_with_presence_public_comment.rb)

## उदाहरण

[inline-code-attrs-start title = 'get_comments_public उदाहरण'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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