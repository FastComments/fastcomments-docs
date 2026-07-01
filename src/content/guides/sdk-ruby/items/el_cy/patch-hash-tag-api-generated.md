## Parameters

| Όνομα | Τύπος | Θέση | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| tag | string | path | Ναι |  |

## Απάντηση

Επιστρέφει: [`UpdateHashTagResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/update_hash_tag_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'patch_hash_tag Παράδειγμα'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Ρύθμιση εξουσιοδότησης κλειδιού API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Απενεργοποιήστε το σχόλιο στην παρακάτω γραμμή για να ορίσετε ένα πρόθεμα για το κλειδί API, π.χ. 'Bearer' (προεπιλογή nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
tag = 'tag_example' # String | 
update_hash_tag_body = FastCommentsClient::UpdateHashTagBody.new # UpdateHashTagBody | 

begin
  
  result = api_instance.patch_hash_tag(tenant_id, tag, update_hash_tag_body)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->patch_hash_tag: #{e}"
end
[inline-code-end]