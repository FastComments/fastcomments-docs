Trenutno online gledatelji stranice: osobe čija je websocket sesija trenutno pretplaćena na stranicu.
Vraća anonCount + totalCount (pretplatnici cijele sobe, uključujući anonimne gledatelje koje ne navodimo).

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (čišćen na strani poslužitelja). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tiebreaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo ispadanje unosa zbog istih imena. |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst koristeći instancu API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (čišćen na strani poslužitelja).
    after_name = 'after_name_example' # str | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
    after_user_id = 'after_user_id_example' # str | Tiebreaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako bi se spriječilo ispadanje unosa zbog istih imena. (opcionalno)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]