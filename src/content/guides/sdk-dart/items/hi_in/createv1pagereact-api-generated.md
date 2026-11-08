## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| title | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: `CreateV1PageReact`

## उदाहरण

[inline-code-attrs-start title = 'createV1PageReact उदाहरण'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | 
final title = title_example; // String | 

try {
    final result = api_instance.createV1PageReact(tenantId, urlId, title);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->createV1PageReact: $e\n');
}
[inline-code-end]