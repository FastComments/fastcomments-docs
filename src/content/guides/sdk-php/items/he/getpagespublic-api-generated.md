List pages for a tenant. Used by the FChat desktop client to populate its room list.  
Requires `enableFChat` to be true on the resolved custom config for each page.  
Pages that require SSO are filtered against the requesting user's group access.

## Parameters

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | מצביע דפדוף אטום שהוחזר כ‑`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`. |
| limit | integer | query | No | 1..200, ברירת מחדל 50 |
| q | string | query | No | מסנן קידומת כותרת רגישות למקרה ואופציונלי. |
| sortBy | string | query | No | סדר מיון. `updatedAt` (ברירת מחדל, החדשות ביותר תחילה), `commentCount` (רוב ההערות תחילה), או `title` (בסדר אלפביתי). |
| hasComments | boolean | query | No | אם true, מחזיר רק דפים עם לפחות הערה אחת. |

## Response

מחזיר: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/GetPublicPagesResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getPagesPublic'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם אתה רוצה להשתמש ב‑client HTTP מותאם, העבר את ה‑client שלך שמממש `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$options = [
    'cursor' => 'cursor_example', // string | מצביע דפדוף אטום שהוחזר כ‑`nextCursor` מבקשה קודמת. קשור לאותו `sortBy`.
    'limit' => 56, // int | 1..200, ברירת מחדל 50
    'q' => 'q_example', // string | מסנן קידומת כותרת רגישות למקרה ואופציונלי.
    'sort_by' => new \FastComments\Client\Model\\FastComments\Client\Model\PagesSortBy(), // \FastComments\Client\Model\PagesSortBy | סדר מיון. `updatedAt` (ברירת מחדל, החדשות ביותר תחילה), `commentCount` (רוב ההערות תחילה), או `title` (בסדר אלפביתי).
    'has_comments' => True, // bool | אם true, מחזיר רק דפים עם לפחות הערה אחת.
];


try {
    $result = $apiInstance->getPagesPublic($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getPagesPublic: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---