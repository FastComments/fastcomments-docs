## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | integer | query | Ne |  |

## Odgovor

Vrne: [`GetSSOUsersResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_sso_users_response.py)

## Primer

[inline-code-attrs-start title = 'Primer get_sso_users'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_sso_users_response import GetSSOUsersResponse
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre overjanja in pooblastil
# v skladu s politiko varnosti API strežnika.
# Spodaj so podani primeri za vsako metodo overjanja; uporabite primer, ki
# ustreza vašemu primeru uporabe overjanja.

# Konfigurirajte overjanje z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnje za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 56 # int |  (neobvezno)

    try:
        api_response = api_instance.get_sso_users(tenant_id, skip=skip)
        print("The response of DefaultApi->get_sso_users:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_sso_users: %s\n" % e)
[inline-code-end]