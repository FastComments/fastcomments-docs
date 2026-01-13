## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| userId | string | query | Όχι |  |
| urlId | string | query | Όχι |  |
| fromCommentId | string | query | Όχι |  |
| viewed | boolean | query | Όχι |  |
| type | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_notification_count200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_notification_count Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Απο-σχολιάστε την παρακάτω γραμμή για να ορίσετε πρόθεμα για το API key, π.χ. 'Bearer' (προεπιλογή: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  user_id: 'user_id_example', # String | 
  url_id: 'url_id_example', # String | 
  from_comment_id: 'from_comment_id_example', # String | 
  viewed: true, # Boolean | 
  type: 'type_example' # String | 
}

begin
  
  result = api_instance.get_notification_count(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_notification_count: #{e}"
end
[inline-code-end]