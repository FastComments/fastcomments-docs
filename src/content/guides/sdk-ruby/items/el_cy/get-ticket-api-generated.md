## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |
| userId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_ticket200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'get_ticket Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης κλειδιού API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Καταργήστε το σχόλιο της παρακάτω γραμμής για να ορίσετε ένα πρόθεμα για το API key, π.χ. 'Bearer' (προεπιλεγμένο nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 
opts = {
  user_id: 'user_id_example' # String | 
}

begin
  
  result = api_instance.get_ticket(tenant_id, id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_ticket: #{e}"
end
[inline-code-end]

---