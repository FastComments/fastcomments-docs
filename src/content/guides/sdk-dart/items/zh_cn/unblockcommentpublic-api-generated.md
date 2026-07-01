## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回：`UnblockSuccess`

## 示例

[inline-code-attrs-start title = 'unBlockCommentPublic 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final commentId = commentId_example; // String | 
final publicBlockFromCommentParams = PublicBlockFromCommentParams(); // PublicBlockFromCommentParams | 
final sso = sso_example; // String | 

try {
    final result = api_instance.unBlockCommentPublic(tenantId, commentId, publicBlockFromCommentParams, sso);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->unBlockCommentPublic: $e\n');
}
[inline-code-end]