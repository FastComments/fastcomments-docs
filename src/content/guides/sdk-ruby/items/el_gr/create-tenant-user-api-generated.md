## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |

## Απόκριση

Επιστρέφει: [`CreateTenantUser200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/create_tenant_user200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα create_tenant_user'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Καταργήστε το σχόλιο στην παρακάτω γραμμή για να ορίσετε πρόθεμα για το API key, π.χ. 'Bearer' (προεπιλογή nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
create_tenant_user_body = FastCommentsClient::CreateTenantUserBody.new({username: 'username_example', email: 'email_example'}) # CreateTenantUserBody | 

begin
  
  result = api_instance.create_tenant_user(tenant_id, create_tenant_user_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->create_tenant_user: #{e}"
end
[inline-code-end]

---