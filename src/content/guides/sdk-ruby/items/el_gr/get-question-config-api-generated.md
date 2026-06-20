## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| id | string | path | Ναι |  |

## Απόκριση

Επιστρέφει: [`GetQuestionConfigResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_config_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_question_config'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Ρύθμιση εξουσιοδότησης μέσω api_key: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Καταργήστε το σχόλιο της επόμενης γραμμής για να ορίσετε ένα πρόθεμα για το κλειδί API, π.χ. 'Bearer' (προεπιλογή: nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
id = 'id_example' # String | 

begin
  
  result = api_instance.get_question_config(tenant_id, id)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_config: #{e}"
end
[inline-code-end]

---