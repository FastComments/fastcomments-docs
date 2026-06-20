## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | putanja | Da |  |
| spam | boolean | upit | Ne |  |
| permNotSpam | boolean | upit | Ne |  |
| sso | string | upit | Ne |  |

## Odgovor

Vraća: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer post_set_comment_spam_status'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je na https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Stvorite instancu API klase
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    spam = True # bool |  (optional)
    perm_not_spam = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.post_set_comment_spam_status(comment_id, spam=spam, perm_not_spam=perm_not_spam, sso=sso)
        print("The response of ModerationApi->post_set_comment_spam_status:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_set_comment_spam_status: %s\n" % e)
[inline-code-end]