## Parametri

| Ime | Tip | Mesto | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| isLive | boolean | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| sendEmails | boolean | query | Ne |  |
| populateNotifications | boolean | query | Ne |  |

## Odziv

Vrne: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## Primer

[inline-code-attrs-start title = 'save_comment Primer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.default_api import SaveCommentOptions
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Definiranje gostitelja je neobvezno in privzeto nastavljeno na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora konfigurirati parametre avtentikacije in avtorizacije
# v skladu s politiko varnosti strežnika API.
# Primeri za vsako metodo avtentikacije so podani spodaj, uporabite primer, ki
# izpolnjuje vaše potrebe avtentikacije.

# Konfigurirajte avtentikacijo s ključem API: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodaj, da nastavite predpono (npr. Bearer) za ključ API, po potrebi
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco odjemalca API
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco API razreda
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (optional)
    do_spam_check = True # bool |  (optional)
    send_emails = True # bool |  (optional)
    populate_notifications = True # bool |  (optional)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, SaveCommentOptions(is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications))
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]