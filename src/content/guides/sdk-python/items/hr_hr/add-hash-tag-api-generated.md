## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ne |  |

## Odgovor

Vraća: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer add_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s pravilima sigurnosti API servera.
# Primjeri za svaku metodu autentifikacije prikazani su dolje; koristite primjer koji
# zadovoljava vaš slučaj uporabe za autentifikaciju.

# Konfigurirajte autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s primjerkom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte primjerak API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (neobavezno)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (neobavezno)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]