## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| usernameStartsWith | string | query | Ne |  |
| mentionGroupIds | array | query | Ne |  |
| sso | string | query | Ne |  |
| searchSection | string | query | Ne |  |

## Odgovor

Vraća: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/search_users200_response.rb)

## Primer

[inline-code-attrs-start title = 'search_users Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
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

---