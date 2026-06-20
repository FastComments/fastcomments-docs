Prethodni komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online kako biste prikazali odjeljak "Članovi".
Straničenje kursora na commenterName: server pregleda parcijalni {tenantId, urlId, commenterName} indeks od afterName unaprijed koristeći $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-a stranice (čišćen na serverskoj strani). |
| afterName | string | query | No | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | No | Kursor za razrješenje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo izostavljanje unosa zbog istih imena. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Primjer

[inline-code-attrs-start title = 'Primjer get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL-a stranice (čišćen na serverskoj strani).
opts = {
  after_name: 'after_name_example', # String | Kursor: proslijedite nextAfterName iz prethodnog odgovora.
  after_user_id: 'after_user_id_example' # String | Kursor za razrješenje neriješenih slučajeva: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo izostavljanje unosa zbog istih imena.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]