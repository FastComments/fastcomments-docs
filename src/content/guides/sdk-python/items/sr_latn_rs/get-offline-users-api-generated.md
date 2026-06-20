Bivši komentatori na stranici koji trenutno NISU online. Sortirano po displayName.
Koristite ovo nakon iscrpljenja /users/online da prikažete sekciju "Članovi".
Cursor paginacija po commenterName: server pregleda parcijalni {tenantId, urlId, commenterName}
indeks od afterName unapred putem $gt, bez troška $skip.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL stranice (obrađen na serverskoj strani). |
| afterName | string | query | Ne | Cursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se zapisi sa istim imenom ne bi izostavili. |

## Odgovor

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL stranice (obrađen na serverskoj strani).
    after_name = 'after_name_example' # str | Cursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
    after_user_id = 'after_user_id_example' # str | Tiebreaker kursora: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se zapisi sa istim imenom ne bi izostavili. (opciono)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]