Prejšnji komentatorji na strani, ki trenutno NISO online. Razvrščeni po displayName.
To uporabite po izčrpanju /users/online za prikaz razdelka "Člani".
Kazalčna paginacija na commenterName: strežnik prehaja delni indeks {tenantId, urlId, commenterName}
od afterName naprej z uporabo $gt, brez stroška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifikator URL-ja strani (očisti se na strežniku). |
| afterName | string | query | No | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | No | Kazalec za razreševanje izenačenosti: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljen, da vnosi pri enakih imenih ne bodo izpuščeni. |

## Odgovor

Vrača: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/page_users_offline_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_offline_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.page_users_offline_response import PageUsersOfflineResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je izbirna in privzeto je https://fastcomments.com
# Glejte configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | Identifikator URL-ja strani (očisti se na strežniku).
    after_name = 'after_name_example' # str | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. (neobvezno)
    after_user_id = 'after_user_id_example' # str | Kazalec za razreševanje izenačenosti: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljen, da vnosi pri enakih imenih ne bodo izpuščeni. (neobvezno)

    try:
        api_response = api_instance.get_offline_users(tenant_id, url_id, after_name=after_name, after_user_id=after_user_id)
        print("The response of PublicApi->get_offline_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_offline_users: %s\n" % e)
[inline-code-end]