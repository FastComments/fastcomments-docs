## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| isLive | boolean | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| sendEmails | boolean | query | Ne |  |
| populateNotifications | boolean | query | Ne |  |

## Odgovor

Vraća: [`SaveComment200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/save_comment200_response.py)

## Primjer

[inline-code-attrs-start title = 'save_comments_bulk Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_comment_params import CreateCommentParams
from client.models.save_comment200_response import SaveComment200Response
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta nije obavezno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Klijent mora konfigurirati parametre autentikacije i autorizacije
# u skladu s politikom sigurnosti API poslužitelja.
# Primjeri za svaki način autentikacije dani su u nastavku, koristite primjer koji
# zadovoljava vaš slučaj upotrebe autentikacije.

# Konfigurirajte autorizaciju putem API ključa: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte dolje kako biste postavili prefiks (npr. Bearer) za API ključ, ako je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = [client.CreateCommentParams()] # List[CreateCommentParams] | 
    is_live = True # bool |  (neobavezno)
    do_spam_check = True # bool |  (neobavezno)
    send_emails = True # bool |  (neobavezno)
    populate_notifications = True # bool |  (neobavezno)

    try:
        api_response = api_instance.save_comments_bulk(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comments_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comments_bulk: %s\n" % e)
[inline-code-end]