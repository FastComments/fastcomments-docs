---
The SDK exposes three API client classes:

- **`DefaultApi`** — サーバーサイドでの使用向けの API キー認証済みメソッド。API キーの設定方法は [はじめに](#getting-started-readme-generated) を参照してください。
- **`PublicApi`** — API キーを必要としない公開メソッドで、ブラウザやモバイルアプリから呼び出しても安全です。
- **`ModerationApi`** — モデレーターダッシュボード向けのメソッド：コメントの一覧表示、カウント、検索、ログ記録およびエクスポート；モデレーション操作（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票、スレッドの再開/クローズ）；バン（コメントからのバン、元に戻す、事前バンのサマリー、バンの状態と設定、バンされたユーザー数）；バッジと信頼度（バッジの付与/削除、手動バッジ、信頼係数の取得/設定、ユーザー内部プロファイル）。すべての `ModerationApi` メソッドは、SSO を介して動作するモデレーターを認証するための `$sso` パラメータを受け取ります。

### PublicApi の使用

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公開メソッドは API キーを必要としません。
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // string
$url_id = 'url_id_example'; // string

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
$sso = 'sso_example'; // string - モデレーターを認証する SSO ペイロード

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---