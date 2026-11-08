## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| batchJobId | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: `ModerationExportStatusResponse`

## उदाहरण

[inline-code-attrs-start title = 'getApiExportStatus उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = ModerationApi();
final tenantId = tenantId_example; // String | 
final batchJobId = batchJobId_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getApiExportStatus(tenantId, GetApiExportStatusOptions(batchJobId: batchJobId, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling ModerationApi->getApiExportStatus: $e\n');
}
[inline-code-end]