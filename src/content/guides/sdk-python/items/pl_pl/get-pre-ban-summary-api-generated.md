## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Tak |  |
| includeByUserIdAndEmail | boolean | query | Nie |  |
| includeByIP | boolean | query | Nie |  |
| includeByEmailDomain | boolean | query | Nie |  |
| sso | string | query | Nie |  |

## Odpowiedź

Zwraca: [`PreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pre_ban_summary.py)

## Przykład

[inline-code-attrs-start title = 'Przykład get_pre_ban_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pre_ban_summary import PreBanSummary
from client.rest import ApiException
from pprint import pprint

# Określenie hosta jest opcjonalne i domyślnie ustawione na https://fastcomments.com
# Zobacz configuration.py, aby uzyskać listę wszystkich obsługiwanych parametrów konfiguracji.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Otwórz kontekst z instancją klienta API
with client.ApiClient(configuration) as api_client:
    # Utwórz instancję klasy API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_by_user_id_and_email = True # bool |  (opcjonalne)
    include_by_ip = True # bool |  (opcjonalne)
    include_by_email_domain = True # bool |  (opcjonalne)
    sso = 'sso_example' # str |  (opcjonalne)

    try:
        api_response = api_instance.get_pre_ban_summary(comment_id, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->get_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_pre_ban_summary: %s\n" % e)
[inline-code-end]