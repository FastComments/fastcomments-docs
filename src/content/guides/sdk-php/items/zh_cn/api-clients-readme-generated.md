---
该 SDK 提供三个 API 客户端类：

- **`DefaultApi`** — 用于服务器端的 API 密钥认证方法。按照 [入门](#getting-started-readme-generated) 中所示配置 API 密钥。
- **`PublicApi`** — 不需要 API 密钥的公共方法，可安全地从浏览器和移动应用调用。
- **`ModerationApi`** — 用于版主仪表板的方法：列出、计数、搜索、记录和导出评论；审核操作（删除/恢复、标记、设置审查/垃圾/批准状态、投票、重新打开/关闭线程）；封禁（从评论中封禁、撤销、预封禁摘要、封禁状态和偏好、被封用户计数）；以及徽章与信任（授予/移除徽章、手动徽章、获取/设置信任因子、用户内部档案）。每个 `ModerationApi` 方法都接受一个 `$sso` 参数，用于通过 SSO 验证执行操作的版主。

### 使用 PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// 公共方法不需要 API 密钥。
$apiInstance = new FastComments\Client\Api\PublicApi(
    new GuzzleHttp\Client()
);
$tenant_id = 'tenant_id_example'; // 字符串
$url_id = 'url_id_example'; // 字符串

try {
    $result = $apiInstance->getCommentsPublic($tenant_id, $url_id);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling PublicApi->getCommentsPublic: ', $e->getMessage(), PHP_EOL;
}
```

### 使用 ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // 字符串 - 验证版主的 SSO 有效载荷

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```
---