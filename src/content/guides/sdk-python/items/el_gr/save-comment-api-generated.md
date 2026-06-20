## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| isLive | boolean | query | Όχι |  |
| doSpamCheck | boolean | query | Όχι |  |
| sendEmails | boolean | query | Όχι |  |
| populateNotifications | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`APISaveCommentResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_save_comment_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα save_comment'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_save_comment_response import APISaveCommentResponse
from client.models.create_comment_params import CreateCommentParams
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και εξ ορισμού είναι https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους αυθεντικοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο αυθεντικοποίησης, χρησιμοποιήστε το παράδειγμα που
# ικανοποιεί την περίπτωση χρήσης αυθεντικοποίησης σας.

# Ρυθμίστε την εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, εάν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Μπείτε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_comment_params = client.CreateCommentParams() # CreateCommentParams | 
    is_live = True # bool |  (προαιρετικό)
    do_spam_check = True # bool |  (προαιρετικό)
    send_emails = True # bool |  (προαιρετικό)
    populate_notifications = True # bool |  (προαιρετικό)

    try:
        api_response = api_instance.save_comment(tenant_id, create_comment_params, is_live=is_live, do_spam_check=do_spam_check, send_emails=send_emails, populate_notifications=populate_notifications)
        print("The response of DefaultApi->save_comment:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->save_comment: %s\n" % e)
[inline-code-end]