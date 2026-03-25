## Parametri

| Name | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da |  |
| usernameStartsWith | string | query | Ne |  |
| mentionGroupIds | array | query | Ne |  |
| sso | string | query | Ne |  |
| searchSection | string | query | Ne |  |

## Odgovor

Vrača: [`SearchUsers200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/search_users200_response.py)

## Primer

[inline-code-attrs-start title = 'search_users Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.search_users200_response import SearchUsers200Response
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 
    username_starts_with = 'username_starts_with_example' # str |  (neobvezno)
    mention_group_ids = ['mention_group_ids_example'] # List[str] |  (neobvezno)
    sso = 'sso_example' # str |  (neobvezno)
    search_section = 'search_section_example' # str |  (neobvezno)

    try:
        api_response = api_instance.search_users(tenant_id, url_id, username_starts_with=username_starts_with, mention_group_ids=mention_group_ids, sso=sso, search_section=search_section)
        print("The response of PublicApi->search_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->search_users: %s\n" % e)
[inline-code-end]