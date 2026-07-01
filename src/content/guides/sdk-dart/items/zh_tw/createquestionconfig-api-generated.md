## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|------|------|
| tenantId | string | query | 是 |  |

## 回應

返回： `CreateQuestionConfigResponse`

## 範例

[inline-code-attrs-start title = 'createQuestionConfig 範例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO 配置 API 金鑰授權：api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 解除註解以下以設定前綴（例如 Bearer）給 API 金鑰，若需要
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final createQuestionConfigBody = CreateQuestionConfigBody(); // CreateQuestionConfigBody | 

try {
    final result = api_instance.createQuestionConfig(tenantId, createQuestionConfigBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->createQuestionConfig: $e\n');
}
[inline-code-end]