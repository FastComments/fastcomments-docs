## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| value | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationCommentSearchResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationCommentSearchResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getSearchCommentsSummary'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$value = 'value_example'; // string
$filters = 'filters_example'; // string
$search_filters = 'search_filters_example'; // string
$sso = 'sso_example'; // string

try {
    $result = $apiInstance->getSearchCommentsSummary($value, $filters, $search_filters, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getSearchCommentsSummary: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]