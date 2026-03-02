### Via Facade

```php
use FastComments\Laravel\Facades\FastComments;

// API de administrador (requer chave de API)
$comments = FastComments::admin()->getComments('tenant-id');

// API pública
$comments = FastComments::publicApi()->getCommentsPublic('tenant-id', 'url-id');

// SSO
$ssoPayload = FastComments::sso()->forWidget();
$token = FastComments::sso()->tokenFor($user);
```

### Via Injeção de Dependência

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

### Acesso Direto ao SDK

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