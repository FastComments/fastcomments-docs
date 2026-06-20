---
רשום דפים עבור שוכר. בשימוש על ידי לקוח שולחני של FChat כדי למלא את רשימת החדרים שלו.
דורש ש- `enableFChat` יהיה true בקונפיג המותאם שנפתר עבור כל דף.
דפים שדורשים SSO מסוננים לפי גישת הקבוצה של המשתמש המבקש.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | בורר עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | No | 1..200, ברירת מחדל 50 |
| q | string | query | No | מסנן אופציונלי של קידומת כותרת שאינו תלוי ברישיות. |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (הכי הרבה תגובות תחילה), או `title` (אלפביתי). |
| hasComments | boolean | query | No | אם true, החזר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם ברצונך להשתמש ב-client HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$cursor = 'cursor_example'; // string | בורר עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
$limit = 56; // int | 1..200, ברירת מחדל 50
$q = 'q_example'; // string | מסנן אופציונלי של קידומת כותרת שאינו תלוי ברישיות.
$sort_by = new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(); // \FastComments\Client\Model\PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדשים ביותר תחילה), `commentCount` (הכי הרבה תגובות תחילה), או `title` (אלפביתי).
$has_comments = True; // bool | אם true, החזר רק דפים עם לפחות תגובה אחת.

try {
    $result = $apiInstance->getPagesPublic($tenant_id, $cursor, $limit, $q, $sort_by, $has_comments);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---