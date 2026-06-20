Trenutno-online posmatrači stranice: osobe čija je websocket sesija pretplaćena na stranicu upravo sada.
Vraća anonCount + totalCount (pretplatnici u okviru sobe, uključujući anonimne posmatrače koje ne navodimo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (očišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: prosledi nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor tiebreaker: prosledi nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako izjednačenja imena (name-ties) ne bi izbacila unose. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Primer

[inline-code-attrs-start title = 'get_online_users Primer'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL-a stranice (očišćen na serverskoj strani).
opts = {
  after_name: 'after_name_example', # String | Kursor: prosledi nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Kursor tiebreaker: prosledi nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako izjednačenja imena (name-ties) ne bi izbacila unose.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]