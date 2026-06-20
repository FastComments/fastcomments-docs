## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| badgeId | string | query | Όχι |  |
| type | number | query | Όχι |  |
| displayedOnComments | boolean | query | Όχι |  |
| limit | number | query | Όχι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APIGetUserBadgesResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/a_p_i_get_user_badges_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_user_badges'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμορφώστε την εξουσιοδότηση με API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Αποσχολιάστε την παρακάτω γραμμή για να ορίσετε ένα πρόθεμα για το API key, π.χ. 'Bearer' (προεπιλογή nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  badge_id: 'badge_id_example', # String | 
  type: 1.2, # Float | 
  displayed_on_comments: true, # Boolean | 
  limit: 1.2, # Float | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_user_badges(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_user_badges: #{e}"
end
[inline-code-end]

---