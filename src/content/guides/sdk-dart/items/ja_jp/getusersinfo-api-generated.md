テナント向けの大量ユーザー情報です。userIds が指定された場合、User / SSOUser から表示情報を返します。コメントウィジェットが、プレゼンスイベントで新たに出現したユーザーを豊かにするために使用されます。ページコンテキストがない場合: プライバシーは一様に適用され（プライベートプロフィールはマスクされます）。

## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| ids | string | query | Yes | カンマ区切りの userIds。 |

## Response

Returns: `PageUsersInfoResponse`

## Example

[inline-code-attrs-start title = 'getUsersInfo の例'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final ids = ids_example; // String | Comma-delimited userIds.

try {
    final result = api_instance.getUsersInfo(tenantId, ids);
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getUsersInfo: $e\n');
}
[inline-code-end]

---