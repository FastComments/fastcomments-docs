Otpremanje i promena veličine slike

## Parameters

| Ime | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Preset veličine: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje) |
| urlId | string | query | No | ID stranice sa koje se vrši otpremanje, za konfiguraciju |

## Response

Vraća: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'uploadImage Primer'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Preset veličine: "Default" (1000x1000px) ili "CrossPlatform" (kreira veličine za popularne uređaje)
final urlId = urlId_example; // String | ID stranice sa koje se vrši otpremanje, za konfiguraciju

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]