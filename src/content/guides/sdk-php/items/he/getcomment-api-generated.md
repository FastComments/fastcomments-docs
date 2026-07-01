## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |

## תגובה

מחזיר: [`APIGetCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIGetCommentResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת getComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// הגדר הרשאת מפתח API: api_key
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
// אם אתה רוצה להשתמש ב‑client http מותאם אישית, העבר את ה‑client שלך המיישם `GuzzleHttp\ClientInterface`.
// זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
$apiInstance = new FastComments\Client\Api\DefaultApi(
    // אם אתה רוצה להשתמש ב‑client http מותאם אישית, העבר את ה‑client שלך המיישם `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string


try {
    $result = $apiInstance->getComment($tenant_id, $id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->getComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]