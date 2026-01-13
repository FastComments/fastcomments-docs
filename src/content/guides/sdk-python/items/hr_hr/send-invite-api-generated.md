## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| fromName | string | query | Da |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer send_invite'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API servera.
# Primjeri za svaku metodu autentikacije prikazani su dolje, koristite primjer koji
# zadovoljava vaš slučaj korištenja autentikacije.

# Configure API key authorization: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Okomentirajte dolje da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    from_name = 'from_name_example' # str | 

    try:
        api_response = api_instance.send_invite(tenant_id, id, from_name)
        print("The response of DefaultApi->send_invite:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->send_invite: %s\n" % e)
[inline-code-end]