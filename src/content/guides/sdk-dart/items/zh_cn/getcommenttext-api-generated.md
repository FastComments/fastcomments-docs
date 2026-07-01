## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| commentId | string | path | 是 |  |
| editKey | string | query | 否 |  |
| sso | string | query | 否 |  |

## 响应

返回: `PublicAPIGetCommentTextResponse`

## 示例

[inline-code-attrs-start title = 'getCommentText 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final editKey = editKey_example; // String | 
final sso = sso_example; // String | 

try {
    final result = api_instance.getCommentText(tenantId, commentId, GetCommentTextOptions(editKey: editKey, sso: sso));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getCommentText: $e\n');
}
[inline-code-end]