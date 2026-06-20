Prošli komentatori na stranici koji trenutno nisu online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da prikažete odjeljak "Članovi".
Paginacija kursorom po commenterName: poslužitelj prolazi parcijalni {tenantId, urlId, commenterName}
indeks od afterName prema naprijed preko $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očisti se na poslužitelju). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tiebreaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Primjer

[inline-code-attrs-start title = 'get_offline_users Example'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Page URL identifier (cleaned server-side).
opts = {
  after_name: 'after_name_example', # String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Tiebreaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Potrebno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]