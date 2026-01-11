## Parametri

| Naziv | Tip | Location | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vraća: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/flag_comment_public200_response.py)

## Primer

[inline-code-attrs-start title = 'Primer update_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.flag_comment_public200_response import FlagCommentPublic200Response
from client.models.pick_api_comment_updatable_comment_fields import PickAPICommentUpdatableCommentFields
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je neobavezno i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati u nastavku, koristite primer koji
# odgovara vašem slučaju upotrebe.

# Konfigurišite autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst sa instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    body = client.PickAPICommentUpdatableCommentFields() # PickAPICommentUpdatableCommentFields | 
    context_user_id = 'context_user_id_example' # str |  (neobavezno)
    do_spam_check = True # bool |  (neobavezno)
    is_live = True # bool |  (neobavezno)

    try:
        api_response = api_instance.update_comment(tenant_id, id, body, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]