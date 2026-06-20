Masovni podaci o korisnicima za tenant. Za zadate userIds, vraća prikazne informacije iz User / SSOUser.
Koristi ga widget za komentare da obogati korisnike koji su se upravo pojavili putem presence događaja.
Nema konteksta stranice: privatnost se primenjuje jednako (privatni profili su maskirani).

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| ids | string | query | Da | Zarezom odvojeni userIds. |

## Odgovor

Vraća: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_info_response.py)

## Primer

[inline-code-attrs-start title = 'get_users_info Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_info_response import PageUsersInfoResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Napravite instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    ids = 'ids_example' # str | Zarezom odvojeni userIds.

    try:
        api_response = api_instance.get_users_info(tenant_id, ids)
        print("The response of PublicApi->get_users_info:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_users_info: %s\n" % e)
[inline-code-end]