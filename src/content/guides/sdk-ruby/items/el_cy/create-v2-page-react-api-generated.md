## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| id | string | query | Ναι |  |
| title | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_v1_page_react.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα create_v2_page_react'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | 
id = 'id_example' # String | 
opts = {
  title: 'title_example' # String | 
}

begin
  
  result = api_instance.create_v2_page_react(tenant_id, url_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->create_v2_page_react: #{e}"
end
[inline-code-end]