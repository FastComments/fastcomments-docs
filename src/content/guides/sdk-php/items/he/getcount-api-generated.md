## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filter | string | query | לא |  |
| searchFilters | string | query | לא |  |
| demo | boolean | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationAPICountCommentsResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationAPICountCommentsResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getCount'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש ב‑client HTTP מותאם, העבר את ה‑client שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$options = [
    'text_search' => 'text_search_example', // מחרוזת
    'by_ip_from_comment' => 'by_ip_from_comment_example', // מחרוזת
    'filter' => 'filter_example', // מחרוזת
    'search_filters' => 'search_filters_example', // מחרוזת
    'demo' => True, // בוליאני
    'sso' => 'sso_example', // מחרוזת
];


try {
    $result = $apiInstance->getCount($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---