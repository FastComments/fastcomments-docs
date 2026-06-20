## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| commentId | string | path | Da |  |
| includeByUserIdAndEmail | boolean | query | Ne |  |
| includeByIP | boolean | query | Ne |  |
| includeByEmailDomain | boolean | query | Ne |  |
| sso | string | query | Ne |  |

## Odgovor

Vrne: [`PreBanSummary`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/pre_ban_summary.py)

## Primer

[inline-code-attrs-start title = 'Primer get_pre_ban_summary'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.pre_ban_summary import PreBanSummary
from client.rest import ApiException
from pprint import pprint

# Določanje gostitelja je neobvezno in privzeto je https://fastcomments.com
# Oglejte si configuration.py za seznam vseh podprtih konfiguracijskih parametrov.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Vstopite v kontekst z instanco odjemalca API-ja
with client.ApiClient(configuration) as api_client:
    # Ustvarite instanco razreda API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    include_by_user_id_and_email = True # bool |  (optional)
    include_by_ip = True # bool |  (optional)
    include_by_email_domain = True # bool |  (optional)
    sso = 'sso_example' # str |  (optional)

    try:
        api_response = api_instance.get_pre_ban_summary(comment_id, include_by_user_id_and_email=include_by_user_id_and_email, include_by_ip=include_by_ip, include_by_email_domain=include_by_email_domain, sso=sso)
        print("The response of ModerationApi->get_pre_ban_summary:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->get_pre_ban_summary: %s\n" % e)
[inline-code-end]