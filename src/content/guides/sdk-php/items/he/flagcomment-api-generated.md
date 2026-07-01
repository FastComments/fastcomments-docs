## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| id | string | path | כן |  |
| userId | string | query | לא |  |
| anonUserId | string | query | לא |  |

## תגובה

מחזיר: [`FlagCommentResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/FlagCommentResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמת flagComment'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');


// // קונפיגורציית אימות מפתח API: api_key
// // בטל את ההערה למטה כדי להגדיר קידומת (לדוגמה Bearer) למפתח ה-API, אם נדרש
// $config = FastComments\Client\Configuration::getDefaultConfiguration()->setApiKeyPrefix('x-api-key', 'Bearer');


$apiInstance = new FastComments\Client\Api\DefaultApi(
    // // אם ברצונך להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך המממש `GuzzleHttp\ClientInterface`.
    // // זה אופציונלי, `GuzzleHttp\Client` ישמש כברירת מחדל.
    new GuzzleHttp\Client(),
    $config
);

$tenant_id = 'tenant_id_example'; // string
$id = 'id_example'; // string
$options = [
    'user_id' => 'user_id_example', // string
    'anon_user_id' => 'anon_user_id_example', // string
];


try {
    $result = $apiInstance->flagComment($tenant_id, $id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling DefaultApi->flagComment: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---