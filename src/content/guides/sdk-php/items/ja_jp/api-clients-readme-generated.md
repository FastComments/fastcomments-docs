The SDKは3つのAPIクライアントクラスを提供します:

- **`DefaultApi`** - サーバー側で使用するためのAPIキー認証メソッドです。APIキーの設定方法は[Getting Started](#getting-started-readme-generated)をご参照ください。
- **`PublicApi`** - APIキーを必要としない公開メソッドで、ブラウザやモバイルアプリから安全に呼び出すことができます。
- **`ModerationApi`** - ライブで高速なモデレーションAPIの包括的なスイートです。すべての `ModerationApi` メソッドは `$sso` パラメータを受け取り、SSOまたは FastComments.com のセッションクッキーで認証できます。

### PublicApi の使用

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公開メソッドは API キーを必要としません。
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 文字列
$url_id = 'url_id_example'; // 文字列

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### ModerationApi の使用

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // 文字列 - モデレーターを認証する SSO ペイロード

try {
    $result = $apiInstance->getCount([
        'sso' => $sso,
    ]);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```