Upload and resize an image

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | Προεπιλογή μεγέθους: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές) |
| urlId | string | query | No | Αναγνωριστικό σελίδας από την οποία γίνεται το ανέβασμα, για ρύθμιση |

## Response

Επιστρέφει: `UploadImageResponse`

## Example

[inline-code-attrs-start title = 'Παράδειγμα uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | Προεπιλογή μεγέθους: "Default" (1000x1000px) ή "CrossPlatform" (δημιουργεί μεγέθη για δημοφιλείς συσκευές)
final urlId = urlId_example; // String | Αναγνωριστικό σελίδας από την οποία γίνεται το ανέβασμα, για ρύθμιση

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Εξαίρεση κατά την κλήση του PublicApi->uploadImage: $e\n');
}
[inline-code-end]