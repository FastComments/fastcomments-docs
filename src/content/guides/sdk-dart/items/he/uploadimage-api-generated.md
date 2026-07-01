העלאה ושינוי גודל של תמונה

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| sizePreset | string | query | לא | תבנית גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים) |
| urlId | string | query | לא | מזהה דף שממנו מתבצעת ההעלאה, להגדרה |

## תגובה

מחזיר: `UploadImageResponse`

## דוגמה

[inline-code-attrs-start title = 'דוגמת uploadImage'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final file = BINARY_DATA_HERE; // MultipartFile | 
final sizePreset = ; // SizePreset | תבנית גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצר גדלים למכשירים פופולריים)
final urlId = urlId_example; // String | מזהה דף שממנו מתבצעת ההעלאה, להגדרה

try {
    final result = api_instance.uploadImage(tenantId, file, UploadImageOptions(sizePreset: sizePreset, urlId: urlId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->uploadImage: $e\n');
}
[inline-code-end]