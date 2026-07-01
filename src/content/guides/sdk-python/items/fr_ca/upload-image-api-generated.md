Upload and resize an image
============================

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Préréglage de taille : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) |
| urlId | string | query | No | Identifiant de page depuis lequel le téléchargement se produit, à configurer |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-python/blob/main/client/models/upload_image_response.py)

## Exemple

[inline-code-attrs-start title = 'upload_image Exemple'; type = 'python'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import client
from client.api.public_api import UploadImageOptions
from client.models.size_preset import SizePreset
from client.models.upload_image_response import UploadImageResponse
from client.rest import ApiException
from pprint import pprint

# La définition de l'hôte est optionnelle et par défaut https://fastcomments.com
# Voir configuration.py pour une liste de tous les paramètres de configuration pris en charge.
configuration = client.Configuration(
    host = "https://fastcomments.com"
)


# Entrer dans un contexte avec une instance du client API
with client.ApiClient(configuration) as api_client:
    # Créer une instance de la classe API
    api_instance = client.PublicApi(api_client)
    tenant_id = 'tenant_id_example' # str | 
    file = None # bytes | 
    size_preset = client.SizePreset() # SizePreset | Préréglage de taille : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) (optionnel)
    url_id = 'url_id_example' # str | Identifiant de page depuis lequel le téléchargement se produit, à configurer (optionnel)

    try:
        api_response = api_instance.upload_image(tenant_id, file, UploadImageOptions(size_preset=size_preset, url_id=url_id))
        print("The response of PublicApi->upload_image:\n")
        pprint(api_response)
    except Exception as e:
        print("Exception when calling PublicApi->upload_image: %s\n" % e)
[inline-code-end]