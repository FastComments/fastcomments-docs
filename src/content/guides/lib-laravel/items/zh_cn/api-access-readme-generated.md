### 通过 Facade

```php
use FastComments\Laravel\Facades\FastComments;

// 管理 API (需要 API 密钥)
$comments = FastComments::admin()->getComments('tenant-id');

// 公共 API
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// 单点登录 (SSO)
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### 通过依赖注入

```php
use FastComments\Laravel\FastCommentsManager;

class CommentController extends Controller
{
    public function index(FastCommentsManager $fc)
    {
        $comments = $fc->admin()->getComments($fc->tenantId());
        // ...
    }
}
```

### 直接访问 SDK

```php
use FastComments\Client\Api\DefaultApi;

class CommentController extends Controller
{
    public function index(DefaultApi $api)
    {
        $comments = $api->getComments('tenant-id');
        // ...
    }
}
```