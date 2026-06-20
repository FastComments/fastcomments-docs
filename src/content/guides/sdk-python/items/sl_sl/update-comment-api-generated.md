## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Da |  |
| id | string | path | Da |  |
| contextUserId | string | query | Ne |  |
| doSpamCheck | boolean | query | Ne |  |
| isLive | boolean | query | Ne |  |

## Odgovor

Vrne: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Primer

[inline-code-attrs-start title = 'Primer update_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.models.updatable_comment_params import UpdatableCommentParams
from client.rest import ApiException
from pprint import pprint

# Določitev gostitelja je neobvezna in privzeto nastavljena na https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Odjemalec mora nastaviti parametre avtentikacije in avtorizacije
# v skladu s politiko varnosti API strežnika.
# Spodaj so prikazani primeri za vsako metodo avtentikacije; uporabite tistega, ki
# ustreza vašemu primeru uporabe avtentikacije.

# Nastavite avtentikacijo z API ključem: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Odkomentirajte spodnjo vrstico za nastavitev predpone (npr. Bearer) za API ključ, če je potrebno
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Vstopite v kontekst z instanco API odjemalca
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    id = 'id_example' # str | 
    updatable_comment_params = client.UpdatableCommentParams() # UpdatableCommentParams | 
    context_user_id = 'context_user_id_example' # str |  (izbirno)
    do_spam_check = True # bool |  (izbirno)
    is_live = True # bool |  (izbirno)

    try:
        api_response = api_instance.update_comment(tenant_id, id, updatable_comment_params, context_user_id=context_user_id, do_spam_check=do_spam_check, is_live=is_live)
        print("The response of DefaultApi->update_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->update_comment: %s\n" % e)
[inline-code-end]