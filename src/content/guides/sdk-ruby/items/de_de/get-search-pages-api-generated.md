## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| value | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Rückgabe: [`ModerationPageSearchResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/moderation_page_search_response.rb)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für get_search_pages'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::ModerationApi.new
tenant_id = 'tenant_id_example' # Zeichenkette |
opts = {
  value: 'value_example', # Zeichenkette |
  sso: 'sso_example' # Zeichenkette |
}

begin
  
  result = api_instance.get_search_pages(tenant_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling ModerationApi->get_search_pages: #{e}"
end
[inline-code-end]