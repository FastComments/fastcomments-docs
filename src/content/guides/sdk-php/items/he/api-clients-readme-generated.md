The SDK מחזיק שלושה מחלקות לקוח של API:

- **`DefaultApi`** - שיטות מאומתות באמצעות מפתח API לשימוש בצד השרת. הגדירו מפתח API כפי שמופיע ב[Getting Started](#getting-started-readme-generated).
- **`PublicApi`** - שיטות ציבוריות שאינן דורשות מפתח API, בטוחות לקריאה מדפדפנים ויישומים ניידים.
- **`ModerationApi`** - מערכת מקיפה של API למודרציה בזמן אמת ומהירה. כל שיטה של `ModerationApi` מקבלת פרמטר `$sso` ויכולה לאמת באמצעות SSO או באמצעות קוביית סשן של FastComments.com.

### שימוש ב‑PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// שיטות ציבוריות אינן דורשות מפתח API.
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // מחרוזת
$url_id = 'url_id_example'; // מחרוזת

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### שימוש ב‑ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // מחרוזת - מטעינת SSO המאמתת את המפקח

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```