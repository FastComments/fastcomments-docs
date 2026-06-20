## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| commentId | string | path | Ναι |  |
| banEmail | boolean | query | Όχι |  |
| banEmailDomain | boolean | query | Όχι |  |
| banIP | boolean | query | Όχι |  |
| deleteAllUsersComments | boolean | query | Όχι |  |
| bannedUntil | string | query | Όχι |  |
| isShadowBan | boolean | query | Όχι |  |
| updateId | string | query | Όχι |  |
| banReason | string | query | Όχι |  |
| sso | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/ban_user_from_comment_result.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα post_ban_user_from_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.ban_user_from_comment_result import BanUserFromCommentResult
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και η προεπιλογή είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρυθμίσεων.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    comment_id = 'comment_id_example' # str | 
    ban_email = True # bool |  (προαιρετικό)
    ban_email_domain = True # bool |  (προαιρετικό)
    ban_ip = True # bool |  (προαιρετικό)
    delete_all_users_comments = True # bool |  (προαιρετικό)
    banned_until = 'banned_until_example' # str |  (προαιρετικό)
    is_shadow_ban = True # bool |  (προαιρετικό)
    update_id = 'update_id_example' # str |  (προαιρετικό)
    ban_reason = 'ban_reason_example' # str |  (προαιρετικό)
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.post_ban_user_from_comment(comment_id, ban_email=ban_email, ban_email_domain=ban_email_domain, ban_ip=ban_ip, delete_all_users_comments=delete_all_users_comments, banned_until=banned_until, is_shadow_ban=is_shadow_ban, update_id=update_id, ban_reason=ban_reason, sso=sso)
        print("The response of ModerationApi->post_ban_user_from_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->post_ban_user_from_comment: %s\n" % e)
[inline-code-end]

---