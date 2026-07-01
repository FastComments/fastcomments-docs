Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.  
Vrne anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).  
Vrne anonCount + totalCount (vseh naročnikov v sobi, vključno z anonimnimi gledalci, ki jih ne izpisujemo).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-ja strani (prečiščen na strani strežnika). |
| afterName | string | query | No | Kazalnik: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kazalnik za razreševanje neodločnosti: posredujte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. |

## Response

Vrne: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_online_response.py)

## Example

[inline-code-attrs-start title = 'Primer get_online_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import GetOnlineUsersOptions
from client.models.page_users_online_response import PageUsersOnlineResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov nastavitve.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvari primerek razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-ja strani (prečiščen na strani strežnika).
    after_name = 'after_name_example' # str | Kazalnik: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
    after_user_id = 'after_user_id_example' # str | Kazalnik za razreševanje neodločnosti: posredujte nextAfterUserId iz prejšnjega odgovora. Potrebno, ko je nastavljen afterName, da se pri enakih imenih ne izpustijo vnosi. (neobvezno)

    try:
        api_response = api_instance.get_online_users(tenant_id, url_id, GetOnlineUsersOptions(after_name=after_name, after_user_id=after_user_id))
        print("The response of PublicApi->get_online_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_online_users: %s\n" % e)
[inline-code-end]