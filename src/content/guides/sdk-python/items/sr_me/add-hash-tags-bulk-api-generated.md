## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ne |  |

## Odgovor

Vraća: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/bulk_create_hash_tags_response.py)

## Primjer

[inline-code-attrs-start title = 'add_hash_tags_bulk Primjer'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.models.bulk_create_hash_tags_body import BulkCreateHashTagsBody
from client.models.bulk_create_hash_tags_response import BulkCreateHashTagsResponse
from client.rest import ApiException
from pprint import pprint

# Definisanje hosta je opciono i podrazumevano je https://fastcomments.com
# Pogledajte configuration.py za listu svih podržanih parametara konfiguracije.
# Klijent mora podesiti parametre autentifikacije i autorizacije
# u skladu sa politikom bezbjednosti API servera.
# Ispod su dati primjeri za svaki metod autentifikacije, upotrijebite primjer koji
# odgovara vašem slučaju upotrebe autentifikacije.
# Podesite autorizaciju API ključa: api_key
# Otkomentarišite ispod da postavite prefiks (npr. Bearer) za API ključ, ako je potrebno
# Uđite u kontekst sa instancom API klijenta
    # Kreirajte instancu API klase
    api_instance = client.DefaultApi(api_client)
    tenant_id = 'tenant_id_example' # str |  (neobavezno)
    bulk_create_hash_tags_body = client.BulkCreateHashTagsBody() # BulkCreateHashTagsBody |  (neobavezno)

    try:
        api_response = api_instance.add_hash_tags_bulk(tenant_id=tenant_id, bulk_create_hash_tags_body=bulk_create_hash_tags_body)
        print("The response of DefaultApi->add_hash_tags_bulk:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling DefaultApi->add_hash_tags_bulk: %s\n" % e)
[inline-code-end]