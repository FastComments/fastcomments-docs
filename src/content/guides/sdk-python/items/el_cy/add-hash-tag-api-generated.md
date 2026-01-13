## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | query | Όχι |  |

## Απόκριση

Επιστρέφει: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/add_hash_tag200_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'Παράδειγμα add_hash_tag'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.add_hash_tag200_response import AddHashTag200Response
from client.models.create_hash_tag_body import CreateHashTagBody
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και έχει προεπιλεγμένη τιμή https://fastcomments.com
# Δείτε το configuration.py για λίστα με όλες τις υποστηριζόμενες παραμέτρους ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)

# Ο client πρέπει να ρυθμίσει τις παραμέτρους πιστοποίησης και εξουσιοδότησης
# σύμφωνα με την πολιτική ασφαλείας του διακομιστή API.
# Παρακάτω παρέχονται παραδείγματα για κάθε μέθοδο εξουσιοδότησης, χρησιμοποιήστε
# το παράδειγμα που καλύπτει την περίπτωση χρήσης πιστοποίησής σας.

# Διαμορφώστε εξουσιοδότηση με API key: api_key
configuration.api_key['api_key'] = os.environ["API_KEY"]

# Αποσχολιάστε παρακάτω για να ορίσετε πρόθεμα (π.χ. Bearer) για το API key, αν χρειάζεται
# configuration.api_key_prefix['api_key'] = 'Bearer'

# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (προαιρετικό)
    create_hash_tag_body = client.CreateHashTagBody() # CreateHashTagBody |  (προαιρετικό)

    try:
        api_response = api_instance.add_hash_tag(tenant_id=tenant_id, create_hash_tag_body=create_hash_tag_body)
        print("The response of DefaultApi->add_hash_tag:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tag: %s\n" % e)
[inline-code-end]