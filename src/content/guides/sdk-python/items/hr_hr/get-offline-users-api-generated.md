Prošli komentatori na stranici koji trenutačno NISU online. Sortirano po displayName.
Koristite ovo nakon što iscrpite /users/online da biste prikazali sekciju 'Članovi'.
Kursor paginacija na commenterName: poslužitelj prolazi djelomični {tenantId, urlId, commenterName} indeks od afterName prema naprijed koristeći $gt, bez troška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Identifikator URL-a stranice (očišćen na strani poslužitelja). |
| afterName | string | query | Ne | Kursor: proslijedite nextAfterName iz prethodnog odgovora. |
| afterUserId | string | query | Ne | Tie-breaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili. |

## Response

Vraća: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-a stranice (očišćen na strani poslužitelja).
    after_name = 'after_name_example' # str | Kursor: proslijedite nextAfterName iz prethodnog odgovora. (opcionalno)
    after_user_id = 'after_user_id_example' # str | Tie-breaker kursora: proslijedite nextAfterUserId iz prethodnog odgovora. Obavezno kada je afterName postavljen kako se unosi s istim imenom ne bi izostavili. (opcionalno)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]