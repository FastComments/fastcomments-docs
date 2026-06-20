## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Response

Vrača: [`GetTenantUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_tenant_user_response.py)

## Example

[inline-code-attrs-start title = 'Primer get_tenant_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_tenant_user_response import GetTenantUserResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in avtorizacije
# v skladu s politiko varnosti strežnika API.
# Spodaj so podani primeri za vsako metodo overjanja, uporabite primer, ki
# ustreza vašemu primeru uporabe overjanja.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.get_tenant_user(tenant_id, id)
        print("The response of DefaultApi->get_tenant_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_tenant_user: %s\n" % e)
[inline-code-end]