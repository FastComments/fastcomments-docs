## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Υποχρεωτικό | Περιγραφή |
|------|------|----------|----------|-------------|
| urlId | string | query | Ναι |  |
| sso | string | query | Όχι |  |

## Απάντηση

Επιστρέφει: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/api_empty_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'put_close_thread Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.api_empty_response import APIEmptyResponse
from client.rest import ApiException
from pprint import pprint

# Ο καθορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους διαμόρφωσης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Χρησιμοποιήστε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.ModerationApi(api_client)
    url_id = 'url_id_example' # str | 
    sso = 'sso_example' # str |  (προαιρετικό)

    try:
        api_response = api_instance.put_close_thread(url_id, sso=sso)
        print("The response of ModerationApi->put_close_thread:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling ModerationApi->put_close_thread: %s\n" % e)
[inline-code-end]