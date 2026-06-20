## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ναι |  |
| urlId | string | query | Ναι |  |
| id | string | query | Ναι |  |

## Απάντηση

Επιστρέφει: [`GetV2PageReactUsersResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_v2_page_react_users_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_v2_page_react_users Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # Συμβολοσειρά | 
url_id = 'url_id_example' # Συμβολοσειρά | 
id = 'id_example' # Συμβολοσειρά | 

begin
  
  result = api_instance.get_v2_page_react_users(tenant_id, url_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_v2_page_react_users: #{e}"
end
[inline-code-end]

---