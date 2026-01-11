Ανέβασμα και αλλαγή μεγέθους εικόνας

## Παράμετροι

| Όνομα | Τύπος | Τοποθεσία | Απαιτείται | Περιγραφή |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Προεπιλογή μεγέθους: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) |
| urlId | string | query | No | ID σελίδας από την οποία γίνεται το ανέβασμα, για διαμόρφωση |

## Απόκριση

Επιστρέφει: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Παράδειγμα

[inline-code-attrs-start title = 'upload_image Παράδειγμα'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# Ορισμός του host είναι προαιρετικός και προεπιλεγμένη τιμή είναι το https://fastcomments.com
# Δείτε το configuration.py για μια λίστα όλων των υποστηριζόμενων παραμέτρων ρύθμισης.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Εισέλθετε σε ένα context με ένα στιγμιότυπο του API client
with client.ApiClient(configuration) as api_client:
    # Δημιουργήστε ένα στιγμιότυπο της κλάσης API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytearray | 
    size_preset = client.SizePreset() # SizePreset | Προεπιλογή μεγέθους: \"Default\" (1000x1000px) ή \"CrossPlatform\" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) (προαιρετικό)
    url_id = 'url_id_example' # str | ID σελίδας από την οποία γίνεται το ανέβασμα, για διαμόρφωση (προαιρετικό)

    try:
        api_response = api_instance.upload_image(tenant_id, file, size_preset=size_preset, url_id=url_id)
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]