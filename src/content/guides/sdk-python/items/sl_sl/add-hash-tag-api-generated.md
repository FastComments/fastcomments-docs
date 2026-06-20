## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ne |  |

## Odgovor

Vrne: [`CreateHashTagResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_hash_tag_response.py)

## Primer

[inline-code-attrs-start title = 'add_hash_tag Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_hash_tag_body import CreateHashTagBody
from client.models.create_hash_tag_response import CreateHashTagResponse
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je izbirno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih parametrov konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre za overjanje in avtorizacijo
# v skladu s varnostno politiko API strežnika.
# Spodaj so podani primeri za vsako metodo overjanja; uporabite primer, ki
# ustreza vašemu primeru uporabe overjanja.

# Konfigurirajte avtorizacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, če želite nastaviti predpono (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (izbirno)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (izbirno)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]