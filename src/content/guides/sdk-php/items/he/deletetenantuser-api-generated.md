## פרמטרים

| שם | סוג | מיקום | נחוץ | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| deleteComments | string | query | No |  |
| commentDeleteMode | string | query | No |  |

## תגובה

מחזירים: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/APIEmptyResponse.php)

## דוגמה

[inline-code-attrs-start title = 'deleteTenantUser דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// Configure API key authorization: api_key
// הגדר הרשאת מפתח API: api_key
// Uncomment below to setup prefix (e.g. Bearer) for API key, if needed
// בטל את ההערה למטה כדי להגדיר קידומת (למשל Bearer) למפתח API, אם נדרש


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // אם אתה רוצה להשתמש בלקוח http מותאם, העבר את הלקוח שלך שמממש `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
// מחרוזת
$id = 'id_example'; // string
// מחרוזת
$options = [
    'delete_comments' => 'delete_comments_example', // string
    // מחרוזת
    'comment_delete_mode' => 'comment_delete_mode_example', // string
    // מחרוזת
];


try {
    $result = $apiInstance->deleteTenantUser($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->deleteTenantUser: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---