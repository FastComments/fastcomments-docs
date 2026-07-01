Upload og ændre størrelsen på et billede

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Størrelsesforudindstilling: "Default" (1000x1000px) eller "CrossPlatform" (opretter størrelser for populære enheder) |
| urlId | string | query | No | Side-id hvor upload sker fra, for konfiguration |

## Response

Returnerer: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'uploadImage Eksempel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Størrelsesforudindstilling: \"Default\" (1000x1000px) eller \"CrossPlatform\" (opretter størrelser for populære enheder)
final urlId = urlId_example; // String | Side-id hvor upload sker fra, for konfiguration

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]