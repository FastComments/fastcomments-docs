## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## レスポンス

戻り値: `APIGetUserBadgesResponse`

## 例

[inline-code-attrs-start title = 'getUserBadges の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';
// TODO APIキー認証を設定してください: api_key
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKey = 'YOUR_API_KEY';
// 必要に応じてAPIキーのプレフィックス（例: Bearer）を設定するには、以下のコメントアウトを解除してください
//defaultApiClient.getAuthentication<ApiKeyAuth>('api_key').apiKeyPrefix = 'Bearer';

final api_instance = DefaultApi();
final tenantId = tenantId_example; // String | 
final userId = userId_example; // String | 
final badgeId = badgeId_example; // String | 
final type = 1.2; // double | 
final displayedOnComments = true; // bool | 
final limit = 1.2; // double | 
final skip = 1.2; // double | 

try {
    final result = api_instance.getUserBadges(tenantId, GetUserBadgesOptions(userId: userId, badgeId: badgeId, type: type, displayedOnComments: displayedOnComments, limit: limit, skip: skip));
    print(result);
} catch (e) {
    print('Exception when calling DefaultApi->getUserBadges: $e\n');
}
[inline-code-end]

---