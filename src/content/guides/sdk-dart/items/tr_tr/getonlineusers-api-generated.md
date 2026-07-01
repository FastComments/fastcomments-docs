Currently-online viewers of a page: people whose websocket session is subscribed to the page right now.
Returns anonCount + totalCount (room-wide subscribers, including anon viewers we don't enumerate).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Sayfa URL kimliği (sunucu tarafında temizlenir). |
| afterName | string | query | No | İmleç: önceki yanıtın nextAfterName değerini gönderin. |
| afterUserId | string | query | No | İmleç eşleme kırıcı: önceki yanıtın nextAfterUserId değerini gönderin. afterName ayarlandığında, isim eşitliklerinin girdileri düşürmemesi için gereklidir. |

## Response

Returns: `PageUsersOnlineResponse`

## Example

[inline-code-attrs-start title = 'getOnlineUsers Örneği'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final urlId = urlId_example; // String | Page URL identifier (cleaned server-side).
final afterName = afterName_example; // String | Cursor: pass nextAfterName from the previous response.
final afterUserId = afterUserId_example; // String | Cursor tiebreaker: pass nextAfterUserId from the previous response. Required when afterName is set so name-ties don't drop entries.

try {
    final result = api_instance.getOnlineUsers(tenantId, urlId, GetOnlineUsersOptions(afterName: afterName, afterUserId: afterUserId));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getOnlineUsers: $e\n');
}
[inline-code-end]