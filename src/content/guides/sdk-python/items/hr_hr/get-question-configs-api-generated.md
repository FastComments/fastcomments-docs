## Parametri

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| skip | number | query | Ne |  |

## Odgovor

Vraća: [`GetQuestionConfigsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_question_configs_response.py)

## Primjer

[inline-code-attrs-start title = 'get_question_configs Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_question_configs_response import GetQuestionConfigsResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je neobavezno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentifikacije su prikazani dolje, koristite primjer koji
# odgovara vašem slučaju uporabe autentifikacije.

# Konfigurirajte autorizaciju pomoću API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Uklonite komentare ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    skip = 3.4 # float |  (neobavezno)

    try:
        api_response = api_instance.get_question_configs(tenant_id, skip=skip)
        print("The response of DefaultApi->get_question_configs:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->get_question_configs: %s\n" % e)
[inline-code-end]