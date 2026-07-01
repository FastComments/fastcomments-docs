## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## 响应

返回: `BlockSuccess`

## 示例

[inline-code-attrs-start title = 'blockUserFromComment 示例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 密钥授权: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 取消注释以下内容以在需要时为 API 密钥设置前缀（例如 Bearer）
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final id = id_example; // String | 
final blockFromCommentParams = BlockFromCommentParams(); // BlockFromCommentParams | 
final userId = userId_example; // String | 
final anonUserId = anonUserId_example; // String | 

try {
    final result = api_instance.blockUserFromComment(tenantId, id, blockFromCommentParams, BlockUserFromCommentOptions(userId: userId, anonUserId: anonUserId));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->blockUserFromComment: $e\n');
}
[inline-code-end]

---