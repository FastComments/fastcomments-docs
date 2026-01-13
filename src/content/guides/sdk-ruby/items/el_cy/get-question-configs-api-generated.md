## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| skip | number | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetQuestionConfigs200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_configs200_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_question_configs'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Διαμόρφωση εξουσιοδότησης με API key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Αφαιρέστε το σχόλιο από την ακόλουθη γραμμή για να ορίσετε ένα πρόθεμα για το API key, π.χ. 'Bearer' (προεπιλογή: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_configs(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_configs: #{e}"
end
[inline-code-end]