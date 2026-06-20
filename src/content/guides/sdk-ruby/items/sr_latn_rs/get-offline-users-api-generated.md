Bivši komentatori na stranici koji trenutno nisu online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da prikažete sekciju "Članovi".
Kursor paginacija po commenterName: server koristi delimični {tenantId, urlId, commenterName} indeks i kreće od afterName unapred pomoću $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL stranice (obrađeno na serverskoj strani). |
| afterName | string | query | No | Kursor: prosledi nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Tiebreaker kursora: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se sprečilo izostavljanje stavki pri istim imenima. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL stranice (obrađeno na serverskoj strani).
opts = {
  after_name: 'after_name_example', # String | Kursor: prosledi nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Tiebreaker kursora: prosledi nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se sprečilo izostavljanje stavki pri istim imenima.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]

---