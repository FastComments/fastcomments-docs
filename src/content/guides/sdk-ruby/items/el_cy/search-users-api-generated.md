## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| usernameStartsWith | string | query | Όχι |  |
| mentionGroupIds | array | query | Όχι |  |
| sso | string | query | Όχι |  |
| searchSection | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα search_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
opts = {
  username_starts_with: 'username_starts_with_example', # String | 
  mention_group_ids: ['inner_example'], # Array<String> | 
  sso: 'sso_example', # String | 
  search_section: 'fast' # String | 
}

begin
  
  result = api_instance.search_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->search_users: #{e}"
end
[inline-code-end]