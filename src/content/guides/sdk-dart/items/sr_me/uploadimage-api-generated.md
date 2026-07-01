Upload and resize an image
==========================

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Predefinisana veličina: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice s koje se vrši otpremanje, za konfiguraciju |

## Odgovor

Vraća: `UploadImageResponse`

## Primjer

[inline-code-attrs-start title = 'uploadImage Primjer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Predefinisana veličina: \"Default\" (1000x1000px) ili \"CrossPlatform\" (kreira veličine za popularne uređaje)
final urlId = urlId_example; // String | ID stranice s koje se vrši otpremanje, za konfiguraciju

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]