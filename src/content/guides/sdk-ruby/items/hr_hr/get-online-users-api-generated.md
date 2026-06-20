Trenutno online gledatelji stranice: osobe čija je WebSocket sesija pretplaćena na stranicu upravo sada.
Vraća anonCount + totalCount (pretplatnici unutar sobe, uključujući anonimne gledatelje koje ne nabrajamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na poslužitelju). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrješavanje izjednačenih imena: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izbačeni. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_online_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer get_online_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL-a stranice (očišćen na poslužitelju).
opts = {
  after_name: 'after_name_example', # String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Kursor za razrješavanje izjednačenih imena: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako unosi s istim imenom ne bi bili izbačeni.
}

begin
  
  result = api_instance.get_online_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_online_users: #{e}"
end
[inline-code-end]

---