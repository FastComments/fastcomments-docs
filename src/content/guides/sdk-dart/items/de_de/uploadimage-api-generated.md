Upload and resize an image
============================

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Größen‑Voreinstellung: "Default" (1000x1000px) oder "CrossPlatform" (erstellt Größen für gängige Geräte) |
| urlId | string | query | No | Seiten‑ID, von der der Upload erfolgt, zur Konfiguration |

## Response

Returns: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'uploadImage Beispiel'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String |
final file = BINARY_DATA_HERE; // MultipartFile |
final sizePreset = ; // SizePreset | Größen‑Voreinstellung: \"Default\" (1000x1000px) oder \"CrossPlatform\" (erstellt Größen für gängige Geräte)
final urlId = urlId_example; // String | Seiten‑ID, von der der Upload erfolgt, zur Konfiguration

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]