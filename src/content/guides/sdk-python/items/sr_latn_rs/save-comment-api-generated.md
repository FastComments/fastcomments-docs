## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| isLive | boolean | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| sendEmails | boolean | query | Ne |  |
| populateNotifications | boolean | query | Ne |  |

## Odgovor

Vraća: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## Primer

[inline-code-attrs-start title = 'Primer save_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je neobavezno i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih konfiguracionih parametara.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora da konfiguriše parametre autentifikacije i autorizacije
# u skladu sa politikom bezbednosti API servera.
# Primeri za svaki metod autentifikacije su dati ispod, koristite primer koji
# zadovoljava vaš slučaj upotrebe.
# Configure API key authorization: api_key
# Otkomentarišite ispod da podesite prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Enter a context with an instance of the API client
with client.ApiClient(configuration) as api_client:
    # Create an instance of the API class
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (neobavezno)
    do_spam_check = True # bool |  (neobavezno)
    send_emails = True # bool |  (neobavezno)
    populate_notifications = True # bool |  (neobavezno)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]

---