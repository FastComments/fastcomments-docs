העלאת תמונה ושינוי גודלה

## Parameters

| שם | סוג | מיקום | חובה | תיאור |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| sizePreset | string | query | No | תבנית גודל: "Default" (1000x1000px) או "CrossPlatform" (יוצרת גדלים למכשירים פופולריים) |
| urlId | string | query | No | מזהה העמוד שממ ממנו מתבצעת העלאה, לצורך קונפיגורציה |

## Response

Returns: [`UploadImageResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/UploadImageResponse.php)

## Example

[inline-code-attrs-start title = 'uploadImage דוגמה'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // אם אתה רוצה להשתמש בלקוח HTTP מותאם, העבר את הלקוח שלך שמיישם `GuzzleHttp\ClientInterface`.
    // זה אופציונלי, `GuzzleHttp\Client` ישמש ברירת מחדל.
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$file = '/path/to/file.txt'; // \SplFileObject
$options = [
    'size_preset' => new \FastComments\Client\Model\\FastComments\Client\Model\SizePreset(), // \FastComments\Client\Model\SizePreset | תבנית גודל: \"Default\" (1000x1000px) או \"CrossPlatform\" (יוצרת גדלים למכשירים פופולריים)
    'url_id' => 'url_id_example', // string | מזהה העמוד שממ ממנו מתבצעת העלאה, לצורך קונפיגורציה
];


try {
    $result = $apiInstance->uploadImage($tenant_id, $file, $options);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->uploadImage: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]

---