Trenutno online gledaoci stranice: ljudi čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici u okviru sobe, uključujući anonimne gledaoce koje ne nabrajamo).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (obrađen na serverskoj strani). |
| afterName | string | query | Ne | Kursor: prosledite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi pri istim imenima stavke ne bi bile izostavljene. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (obrađen na serverskoj strani).
    after_name = 'after_name_example' # str | Kursor: prosledite nextAfterName iz prethodnog odgovora. (opciono)
    after_user_id = 'after_user_id_example' # str | Kursor za razrešavanje izjednačenja: prosledite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi pri istim imenima stavke ne bi bile izostavljene. (opciono)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]