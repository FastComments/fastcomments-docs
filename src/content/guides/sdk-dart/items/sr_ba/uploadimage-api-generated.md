Upload i promjena veličine slike

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Postavka veličine: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice s koje se vrši upload, za konfiguraciju |

## Response

Vraća: `UploadImageResponse`

## Primjer

[inline-code-attrs-start title = 'Primjer uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Postavka veličine: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje)
final urlId = urlId_example; // String | ID stranice s koje se vrši upload, za konfiguraciju

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]

---