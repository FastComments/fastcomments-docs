## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | upit | Da |  |
| id | string | put | Da |  |
| deleteComments | boolean | upit | Ne |  |
| commentDeleteMode | string | upit | Ne |  |

## Odgovor

Vraća: [`DeleteSSOUserAPIResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/delete_sso_user_api_response.py)

## Primer

[inline-code-attrs-start title = 'delete_sso_user Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import DeleteSsoUserOptions
from client.models.delete_sso_user_api_response import DeleteSSOUserAPIResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumeva https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa sigurnosnom politikom API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# odgovara vašem slučaju upotrebe autentifikacije.

# Konfigurišite autorizaciju API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    delete_comments = True # bool |  (opcionalno)
    comment_delete_mode = 'comment_delete_mode_example' # str |  (opcionalno)

    try:
        api_response = api_instance.delete_sso_user(tenant_id, id, DeleteSsoUserOptions(delete_comments=delete_comments, comment_delete_mode=comment_delete_mode))
        print("Odgovor DefaultApi->delete_sso_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Izuzetak prilikom pozivanja DefaultApi->delete_sso_user: %s\n" % e)
[inline-code-end]