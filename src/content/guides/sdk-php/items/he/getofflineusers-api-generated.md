מגיבים בעבר בדף שאינם מחוברים כרגע. ממוינים לפי displayName.
השתמשו בזה לאחר אזילת /users/online כדי להציג מקטע "Members".
עימוד באמצעות סמן על commenterName: השרת מתקדם לאורך האינדקס החלקי {tenantId, urlId, commenterName}
מ-afterName קדימה באמצעות $gt, ללא עלות $skip.

## פרמטרים

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | כן |  |
| urlId | string | query | כן | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת). |
| afterName | string | query | לא | סמן: העבירו את nextAfterName מהתשובה הקודמת. |
| afterUserId | string | query | לא | שובר שוויון לסמן: העבירו את nextAfterUserId מהתשובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות. |

## תשובה

מחזיר: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersOfflineResponse.php)

## דוגמה

[inline-code-attrs-start title = 'דוגמה ל-getOfflineUsers'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // If you want use custom http client, pass your client which implements `GuzzleHttp\ClientInterface`.
    // This is optional, `GuzzleHttp\Client` will be used as default.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string | מזהה כתובת ה-URL של הדף (מנוקה בצד השרת).
$options = [
    'after_name' => 'after_name_example', // string | סמן: העבירו את nextAfterName מהתשובה הקודמת.
    'after_user_id' => 'after_user_id_example', // string | שובר שוויון לסמן: העבירו את nextAfterUserId מהתשובה הקודמת. נדרש כאשר afterName מוגדר כדי ששמות זהים לא יגרמו להשמטת רשומות.
];


try {
    $result = $apiInstance->getOfflineUsers($tenant_id, $url_id, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getOfflineUsers: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]