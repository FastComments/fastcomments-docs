Upload en wijzig de grootte van een afbeelding

## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| sizePreset | string | query | Nee | Groottevoorkeur: "Default" (1000x1000px) of "CrossPlatform" (maakt groottes voor populaire apparaten) |
| urlId | string | query | Nee | Pagina-id waarvan de upload plaatsvindt, om te configureren |

## Response

Retourneert: `UploadImageResponse`

## Voorbeeld

[inline-code-attrs-start title = 'uploadImage Voorbeeld'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Groottevoorkeur: \"Default\" (1000x1000px) of \"CrossPlatform\" (maakt groottes voor populaire apparaten)
final urlId = urlId_example; // String | Pagina-id waarvan de upload plaatsvindt, om te configureren

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]