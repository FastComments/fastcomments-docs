ה-SDK מציע שלוש מחלקות לקוח API:

- **`DefaultApi`** — שיטות מאומתות באמצעות מפתח API לשימוש בצד השרת. הגדר מפתח API כפי שמוצג ב[התחלת עבודה](#getting-started-readme-generated).
- **`PublicApi`** — שיטות ציבוריות שאינן דורשות מפתח API, ובטוחות לקריאה מדפדפנים ויישומי מובייל.
- **`ModerationApi`** — שיטות ללוח הבקרה של המודרטורים: רישום, ספירה, חיפוש, רישום וייצוא תגובות; פעולות מודרציה (הסרה/שחזור, דיווח, קביעת סטטוס לסקירה/ספאם/אישור, הצבעות, פתיחה/סגירת אשכול); חסימות (חסימה מתגובה, ביטול, סיכומים לפני חסימה, מצב והעדפות חסימה, ספירת משתמשים חסומים); ותגים ואמון (הענקה/הסרה של תג, תגי ידני, קבלת/הגדרת גורם אמון, פרופיל פנימי של משתמש). כל שיטה ב-`ModerationApi` מקבלת פרמטר `$sso` לאימות המודרטור המבצע דרך SSO.

### שימוש ב-PublicApi

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

### שימוש ב-ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // מחרוזת - מטען SSO המאמת את המודרטור

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```