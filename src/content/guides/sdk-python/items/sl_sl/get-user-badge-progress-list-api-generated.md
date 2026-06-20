## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| userId | string | query | Ne |  |
| limit | number | query | Ne |  |
| skip | number | query | Ne |  |

## Odgovor

Vrne: [`APIGetUserBadgeProgressListResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_get_user_badge_progress_list_response.py)

## Primer

[inline-code-attrs-start title = 'get_user_badge_progress_list Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_get_user_badge_progress_list_response import APIGetUserBadgeProgressListResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s varnostno politiko API strežnika.
# Primeri za vsako metodo avtentikacije so podani spodaj; uporabite primer, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje, da nastavite predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    user_id = 'user_id_example' # str |  (optional)
    limit = 3.4 # float |  (optional)
    skip = 3.4 # float |  (optional)

    try:
        api_response = api_instance.get_user_badge_progress_list(tenant_id, user_id=user_id, limit=limit, skip=skip)
        print("The response of DefaultApi->get_user_badge_progress_list:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_user_badge_progress_list: %s\n" % e)
[inline-code-end]

---