## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| value | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## דוגמה

[inline-code-attrs-start title = 'getSearchCommentsSummary דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // מחרוזת
$options = [
    'value' => 'value_example', // מחרוזת
    'filters' => 'filters_example', // מחרוזת
    'search_filters' => 'search_filters_example', // מחרוזת
    'sso' => 'sso_example', // מחרוזת
];


try {
    $result = $apiInstance->getSearchCommentsSummary($tenant_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---