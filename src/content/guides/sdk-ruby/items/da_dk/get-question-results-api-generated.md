## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| urlId | string | query | Nej |  |
| userId | string | query | Nej |  |
| startDate | string | query | Nej |  |
| questionId | string | query | Nej |  |
| questionIds | string | query | Nej |  |
| skip | number | query | Nej |  |

## Svar

Returnerer: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/get_question_results200_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_question_results Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'
# opsæt autorisation
FastCommentsClient.configure do |config|
  # Konfigurer API-nøgleautorisation: api_key
  config.api_key['x-api-key'] = 'YOUR API KEY'
  # Fjern kommentaren på følgende linje for at sætte en præfiks for API-nøglen, f.eks. 'Bearer' (standard er nil)
  # config.api_key_prefix['x-api-key'] = 'Bearer'
end

api_instance = FastCommentsClient::DefaultApi.new
tenant_id = 'tenant_id_example' # String | 
opts = {
  url_id: 'url_id_example', # String | 
  user_id: 'user_id_example', # String | 
  start_date: 'start_date_example', # String | 
  question_id: 'question_id_example', # String | 
  question_ids: 'question_ids_example', # String | 
  skip: 1.2 # Float | 
}

begin
  
  result = api_instance.get_question_results(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling DefaultApi->get_question_results: #{e}"
end
[inline-code-end]