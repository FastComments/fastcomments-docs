מאחזר דפים עבור טננט. משמש את לקוח שולחן העבודה FChat לאכלוס רשימת החדרים שלו.
דורש ש-`enableFChat` יהיה true בתצורה המותאמת שנפתרה עבור כל דף.
דפים הדורשים SSO מסוננים מול הרשאות הקבוצה של המשתמש המבקש.

## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| cursor | string | query | לא | סמן עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | לא | 1..200, ברירת מחדל 50 |
| q | string | query | לא | מסנן תחילית כותרת אופציונלי, לא רגיש לרישיות. |
| sortBy | string | query | לא | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (הכי הרבה תגובות ראשון), או `title` (אלפביתי). |
| hasComments | boolean | query | לא | אם true, מחזיר רק דפים עם לפחות תגובה אחת. |

## תגובה

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | סמן עימוד אטום שהוחזר כ-`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
    'limit' => 56, // int | 1..200, ברירת מחדל 50
    'q' => 'q_example', // string | מסנן תחילית כותרת אופציונלי, לא רגיש לרישיות.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדש ביותר ראשון), `commentCount` (הכי הרבה תגובות ראשון), או `title` (אלפביתי).
    'has_comments' => True, // bool | אם true, מחזיר רק דפים עם לפחות תגובה אחת.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]