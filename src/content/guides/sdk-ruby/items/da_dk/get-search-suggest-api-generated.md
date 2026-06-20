## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| text-search | string | query | Nej |  |
| sso | string | query | Nej |  |

## Respons

Returnerer: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_suggest_response.rb)

## Eksempel

[inline-code-attrs-start title = 'get_search_suggest Eksempel'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
opts = {
  text_search: 'text_search_example', # String | 
  sso: 'sso_example' # String | 
}

begin
  
  result = api_instance.get_search_suggest(opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_suggest: #{e}"
end
[inline-code-end]

---