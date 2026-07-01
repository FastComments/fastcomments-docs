## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| title | string | query | No |  |

## 回應

Returns: `CreateV1PageReact`

## 示例

[inline-code-attrs-start title = 'createV1PageReact 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
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