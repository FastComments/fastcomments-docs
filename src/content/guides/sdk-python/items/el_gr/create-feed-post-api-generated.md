## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ναι |  |
| broadcastId | string | query | Όχι |  |
| isLive | boolean | query | Όχι |  |
| doSpamCheck | boolean | query | Όχι |  |
| skipDupCheck | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`CreateFeedPostsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/create_feed_posts_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα create_feed_post'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.create_feed_post_params import CreateFeedPostParams
from client.models.create_feed_posts_response import CreateFeedPostsResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλογή το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να διαμορφώσει τις παραμέτρους authentication και authorization
# σύμφωνα με την πολιτική ασφάλειας του API server.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο auth, χρησιμοποιήστε το παράδειγμα που
# καλύπτει την περίπτωση χρήσης auth σας.

# Διαμόρφωση της εξουσιοδότησης με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Απο-σχολιάστε παρακάτω για να ρυθμίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα instance του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα instance της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    create_feed_post_params = client.CreateFeedPostParams() # CreateFeedPostParams | 
    broadcast_id = 'broadcast_id_example' # str |  (προαιρετικό)
    is_live = True # bool |  (προαιρετικό)
    do_spam_check = True # bool |  (προαιρετικό)
    skip_dup_check = True # bool |  (προαιρετικό)

    try:
        api_response = api_instance.create_feed_post(tenant_id, create_feed_post_params, broadcast_id=broadcast_id, is_live=is_live, do_spam_check=do_spam_check, skip_dup_check=skip_dup_check)
        print("The response of DefaultApi->create_feed_post:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->create_feed_post: %s\n" % e)
[inline-code-end]