Momenteel online kijkers van een pagina: mensen wiens websocket-sessie momenteel op de pagina geabonneerd is.
Retourneert anonCount + totalCount (kamerbrede abonnees, inclusief anonieme kijkers die we niet opsommen).

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificator (geschoond aan serverzijde). |
| afterName | string | query | Nee | Cursor: geef nextAfterName door uit het vorige antwoord. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items verloren gaan. |

## Respons

Retourneert: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Voorbeeld

[inline-code-attrs-start title = 'get_online_users Voorbeeld'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Pagina-URL-identificator (geschoond aan de serverzijde).
opts = {
  after_name: 'after_name_example', # String | Cursor: geef nextAfterName door uit het vorige antwoord.
  after_user_id: 'after_user_id_example' # String | Cursor tiebreaker: geef nextAfterUserId door uit het vorige antwoord. Vereist wanneer afterName is ingesteld zodat bij gelijke namen geen items verloren gaan.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---