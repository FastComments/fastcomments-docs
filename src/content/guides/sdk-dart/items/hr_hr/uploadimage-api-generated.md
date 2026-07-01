Upload and resize an image

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | path | Da |  |
| sizePreset | string | query | Ne | Veličina predložak: "Default" (1000x1000px) ili "CrossPlatform" (stvara veličine za popularne uređaje) |
| urlId | string | query | Ne | ID stranice s koje se vrši učitavanje, za konfiguriranje |

## Response

Returns: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'Primjer uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Veličina predložak: \"Default\" (1000x1000px) ili \"CrossPlatform\" (stvara veličine za popularne uređaje)
final urlId = urlId_example; // String | ID stranice s koje se vrši učitavanje, za konfiguriranje

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]