---
Téléverser et redimensionner une image

## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|-------------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Préréglage de taille : "Default" (1000x1000px) ou "CrossPlatform" (crée des tailles pour les appareils populaires) |
| urlId | string | query | No | Identifiant de page depuis lequel le téléversement a lieu, à configurer |

## Réponse

Retourne : `UploadImageResponse`

## Exemple

[inline-code-attrs-start title = 'uploadImage Exemple'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Préréglage de taille : \"Default\" (1000x1000px) ou \"CrossPlatform\" (crée des tailles pour les appareils populaires)
final urlId = urlId_example; // String | Identifiant de page depuis lequel le téléversement a lieu, à configurer

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]

---