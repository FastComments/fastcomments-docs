Prejšnji komentatorji na strani, ki trenutno NISO na spletu. Razvrščeni po displayName.
Uporabite to po izčrpanju /users/online za izris razdelka 'Člani'.
Straničenje z kazalom na commenterName: strežnik prehodi delni {tenantId, urlId, commenterName} indeks od afterName naprej z uporabo $gt, brez stroška $skip.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL strani (očiščen na strežniški strani). |
| afterName | string | query | No | Kazalec: pošljite nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Veznik za izenačitve kazala: pošljite nextAfterUserId iz prejšnjega odgovora. Zahtevan, kadar je afterName nastavljen, da vnosi z enakimi imeni ne bodo izpuščeni. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-ruby/blob/master/client/lib/fastcomments-client/models/page_users_offline_response.rb)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'ruby'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
require 'time'
require 'fastcomments-client'

api_instance = FastCommentsClient::PublicApi.new
tenant_id = 'tenant_id_example' # String | 
url_id = 'url_id_example' # String | Identifikator URL strani (očiščen na strežniški strani).
opts = {
  after_name: 'after_name_example', # String | Kazalec: pošljite nextAfterName iz prejšnjega odgovora.
  after_user_id: 'after_user_id_example' # String | Veznik za izenačitve kazala: pošljite nextAfterUserId iz prejšnjega odgovora. Zahtevan, kadar je afterName nastavljen, da vnosi z enakimi imeni ne bodo izpuščeni.
}

begin
  
  result = api_instance.get_offline_users(tenant_id, url_id, opts)
  p result
rescue FastCommentsClient::ApiError => e
  puts "Error when calling PublicApi->get_offline_users: #{e}"
end
[inline-code-end]