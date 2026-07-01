## פרמטרים

| שם | סוג | מיקום | נדרש | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |
| anonUserId | string | query | No |  |

## תגובה

מחזיר: [`UnblockSuccess`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UnblockSuccess.php)

## דוגמה

[inline-code-attrs-start title = 'unBlockUserFromComment דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הגדר הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש
// If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
// אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
// This is optional, `GuzzleHttp\Client` will be used as default.
// זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש ברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// string
$id = 'id_example'; // string
// string
$un_block_from_comment_params = new \FastComments\Client\Model\UnBlockFromCommentParams(); // \FastComments\Client\Model\UnBlockFromCommentParams
$options = [
    'user_id' => 'user_id_example', // string
    // מחרוזת
    'anon_user_id' => 'anon_user_id_example', // string
    // מחרוזת
];


try {
    $result = $apiInstance->unBlockUserFromComment($tenant_id, $id, $un_block_from_comment_params, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->unBlockUserFromComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]