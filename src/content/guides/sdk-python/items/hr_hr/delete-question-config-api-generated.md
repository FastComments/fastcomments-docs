## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer delete_question_config'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih konfiguracijskih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentifikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaku metodu autentifikacije dati su dolje; upotrijebite primjer koji
# odgovara vašem scenariju upotrebe autentifikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 

    try:
        api_response = api_instance.delete_question_config(tenant_id, id)
        print("The response of DefaultApi->delete_question_config:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->delete_question_config: %s\n" % e)
[inline-code-end]