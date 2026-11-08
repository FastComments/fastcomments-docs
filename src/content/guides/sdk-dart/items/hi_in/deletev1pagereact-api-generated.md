## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: `CreateV1PageReact`

## उदाहरण

[inline-code-attrs-start title = 'deleteV1PageReact उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 

try {
    final result = api_instance.deleteV1PageReact(tenantId, urlId);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->deleteV1PageReact: $e\n');
}
[inline-code-end]