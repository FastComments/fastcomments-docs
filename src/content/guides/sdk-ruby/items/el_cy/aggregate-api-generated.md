Συγκεντρώνει έγγραφα ομαδοποιώντας τα (αν παρέχεται το groupBy) και εφαρμόζοντας πολλαπλές λειτουργίες.
Υποστηρίζονται διαφορετικές λειτουργίες (π.χ. sum, countDistinct, avg, κ.λπ.).

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| parentTenantId | string | query | Όχι |  |
| includeStats | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AggregateResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/aggregate_response.rb)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα aggregate'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# ρύθμιση εξουσιοδότησης
FastCommentsClient.configure do |config|
  # Ρύθμιση εξουσιοδότησης με κλειδί API: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Αποσχολιάστε την ακόλουθη γραμμή για να ορίσετε ένα πρόθεμα για το κλειδί API, π.χ. 'Bearer' (εξ ορισμού nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
aggregation_request = FastCommentsClient::AggregationRequest.new({resource_name: 'resource_name_example', operations: [FastCommentsClient::AggregationOperation.new({field: 'field_example', op: FastCommentsClient::AggregationOpType::SUM})]}) # AggregationRequest | 
opts = {
  parent_tenant_id: 'parent_tenant_id_example', # String | 
  include_stats: true # Boolean | 
}

begin
  
  result = api_instance.aggregate(tenant_id, aggregation_request, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->aggregate: #{e}"
end
[inline-code-end]