テナントのユーザー情報を一括取得します。userId を指定すると、User / SSOUser から表示情報を返します。
コメントウィジェットが、プレゼンスイベントで直後に現れたユーザーを補完するために使用します。
ページのコンテキストはありません: プライバシーは一律に適用されます(非公開プロフィールはマスクされます)。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| ids | string | query | はい | カンマ区切りの userIds。 |

## レスポンス

戻り値: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-php/blob/main/lib/Model/PageUsersInfoResponse.php)

## 例

[inline-code-attrs-start title = 'getUsersInfo の例'; type = 'php'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<?php
require_once(__DIR__ . '/vendor/autoload.php');



$apiInstance = new FastComments\Client\Api\PublicApi(
    // カスタム HTTP クライアントを使用したい場合は、`GuzzleHttp\ClientInterface` を実装したクライアントを渡してください。
    // これは省略可能で、デフォルトとして `GuzzleHttp\Client` が使用されます。
    new GuzzleHttp\Client()
);

$tenant_id = 'tenant_id_example'; // string
$ids = 'ids_example'; // string | カンマ区切りの userIds。


try {
    $result = $apiInstance->getUsersInfo($tenant_id, $ids);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getUsersInfo: ', $e->getMessage(), PHP_EOL;
}
[inline-code-end]