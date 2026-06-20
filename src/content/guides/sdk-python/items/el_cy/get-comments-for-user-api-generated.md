## Παράμετροι

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| userId | string | query | Όχι |  |
| direction | string | query | Όχι |  |
| repliesToUserId | string | query | Όχι |  |
| page | number | query | Όχι |  |
| includei10n | boolean | query | Όχι |  |
| locale | string | query | Όχι |  |
| isCrawler | boolean | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`GetCommentsForUserResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/get_comments_for_user_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα get_comments_for_user'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.get_comments_for_user_response import GetCommentsForUserResponse
from client.models.sort_directions import SortDirections
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλέγεται σε https://fastcomments.com
# Δείτε το configuration.py για μια λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Ανοίξτε ένα context με ένα στιγμιότυπο του πελάτη API
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    user_id = 'user_id_example' # str |  (προαιρετικό)
    direction = client.SortDirections() # SortDirections |  (προαιρετικό)
    replies_to_user_id = 'replies_to_user_id_example' # str |  (προαιρετικό)
    page = 3.4 # float |  (προαιρετικό)
    includei10n = True # bool |  (προαιρετικό)
    locale = 'locale_example' # str |  (προαιρετικό)
    is_crawler = True # bool |  (προαιρετικό)

    try:
        api_response = api_instance.get_comments_for_user(user_id=user_id, direction=direction, replies_to_user_id=replies_to_user_id, page=page, includei10n=includei10n, locale=locale, is_crawler=is_crawler)
        print("The response of PublicApi->get_comments_for_user:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->get_comments_for_user: %s\n" % e)
[inline-code-end]