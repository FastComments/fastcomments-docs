List pages for a tenant. Used by the FChat desktop client to populate its room list.  
דורש שהמאפיין `enableFChat` יהיה אמת (true) בתצורת המותאמת שנפתרה עבור כל דף.  
דפים הדורשים SSO מסוננים בהתאם לגישת קבוצת המשתמש המבקש.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | סמן דפדוף אטום שמוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | No | 1..200, ברירת מחדל 50 |
| q | string | query | No | מסנן קידומת כותרת רגישות לאותיות גדולות/קטנות (case-insensitive) באופן אופציונלי. |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (התגובות הרבות ביותר ראשונות), או `title` (אלפביתי). |
| hasComments | boolean | query | No | אם אמת, תחזיר רק דפים עם לפחות תגובה אחת. |

## Response

Returns: `GetPublicPagesResponse`

## Example

[inline-code-attrs-start title = 'דוגמה getPagesPublic'; type = ''; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
import 'package:fastcomments_dart/api.dart';

final api_instance = PublicApi();
final tenantId = tenantId_example; // String | 
final cursor = cursor_example; // String | סמן דפדוף אטום שמוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
final limit = 56; // int | 1..200, default 50
final q = q_example; // String | מסנן קידומת כותרת רגישות לאותיות גדולות/קטנות (case-insensitive) באופן אופציונלי.
final sortBy = ; // PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (התגובות הרבות ביותר ראשונות), או `title` (אלפביתי).
final hasComments = true; // bool | אם אמת, תחזיר רק דפים עם לפחות תגובה אחת.

try {
    final result = api_instance.getPagesPublic(tenantId, GetPagesPublicOptions(cursor: cursor, limit: limit, q: q, sortBy: sortBy, hasComments: hasComments));
    print(result);
} catch (e) {
    print('Exception when calling PublicApi->getPagesPublic: $e\n');
}
[inline-code-end]