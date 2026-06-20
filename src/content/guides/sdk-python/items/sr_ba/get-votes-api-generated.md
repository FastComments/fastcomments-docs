## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Да |  |

## Одговор

Враћа: [`GetVotesResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_votes_response.py)

## Пример

[inline-code-attrs-start title = 'get_votes Пример'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_votes_response import GetVotesResponse
from client.rest import ApiException
from pprint import pprint

# Постављање host-а је опционално и подразумевано је https://fastcomments.com
# See configuration.py for a list of all supported configuration parameters.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Клијент мора конфигурисати параметре аутентификације и овлашћења
# у складу са безбедносном политиком API сервера.
# Испод су примери за сваки метод аутентификације, користите онај
# који одговара вашем случају употребе аутентификације.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Ако је потребно, откоментаришите испод да подесите префикс (нпр. Bearer) за API key
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Уђите у контекст са инстанцом API клијента
with client.ApiClient(configuration) as api_client:
    # Креирајте инстанцу API класе
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    url_id = 'url_id_example' # str | 

    try:
        api_response = api_instance.get_votes(tenant_id, url_id)
        print("The response of DefaultApi->get_votes:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_votes: %s\n" % e)
[inline-code-end]