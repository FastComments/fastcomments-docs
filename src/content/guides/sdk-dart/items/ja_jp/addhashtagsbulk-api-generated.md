## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |

## 応答

戻り値: `BulkCreateHashTagsResponse`

## 例

[inline-code-attrs-start title = 'addHashTagsBulk 例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 以下のコメントを外してプレフィックスを設定 (例: Bearer) APIキー用、必要な場合
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final bulkCreateHashTagsBody = BulkCreateHashTagsBody(); // BulkCreateHashTagsBody | 

try {
    final result = api_instance.addHashTagsBulk(tenantId, bulkCreateHashTagsBody);
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->addHashTagsBulk: $e\n');
}
[inline-code-end]