## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| text-search | string | query | לא |  |
| byIPFromComment | string | query | לא |  |
| filters | string | query | לא |  |
| searchFilters | string | query | לא |  |
| sorts | string | query | לא |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/ModerationExportResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-postApiExport'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\ModerationApi(
    // אם ברצונך להשתמש בלקוח HTTP מותאם אישית, העבר את הלקוח שלך שמממש את `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client()
);
$text_search = 'text_search_example'; // מחרוזת
$by_ip_from_comment = 'by_ip_from_comment_example'; // מחרוזת
$filters = 'filters_example'; // מחרוזת
$search_filters = 'search_filters_example'; // מחרוזת
$sorts = 'sorts_example'; // מחרוזת
$sso = 'sso_example'; // מחרוזת

try {
    $result = $apiInstance->postApiExport($text_search, $by_ip_from_comment, $filters, $search_filters, $sorts, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->postApiExport: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]