## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comment_text_response.py)

## Primjer

[inline-code-attrs-start title = 'Primjer get_moderation_comment_text'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comment_text_response import GetCommentTextResponse
from client.rest import ApiException
from pprint import pprint

# Definiranje hosta je opcionalno i zadano je https://fastcomments.com
# Pogledajte configuration.py za popis svih podržanih parametara konfiguracije.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Uđite u kontekst s instancom API klijenta
with client.ApiClient(configuration) as api_client:
    # Kreirajte instancu API klase
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    sso = 'sso_example' # str |  (neobavezno)

    try:
        api_response = api_instance.get_moderation_comment_text(comment_id, sso=sso)
        print("The response of ModerationApi->get_moderation_comment_text:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_moderation_comment_text: %s\n" % e)
[inline-code-end]

---