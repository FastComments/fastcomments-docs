The SDK exposes three API client classes:

- **`DefaultApi`** — API-key-authenticated methods for server-side use. Configure an API key as shown in [Getting Started](#getting-started-readme-generated).
- **`PublicApi`** — public methods that do not require an API key, safe to call from browsers and mobile apps.
- **`ModerationApi`** — methods for the moderator dashboard: listing, counting, searching, logging and exporting comments; moderation actions (remove/restore, flag, set review/spam/approval status, votes, reopen/close thread); bans (ban from comment, undo, pre-ban summaries, ban status and preferences, banned-user counts); and badges & trust (award/remove badge, manual badges, get/set trust factor, user internal profile). Every `ModerationApi` method accepts an `$sso` parameter to authenticate the acting moderator via SSO.

### Using PublicApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

// Public methods do not require an API key.
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

### Using ModerationApi

```php
<?php
require_once(__DIR__ . '/vendor/autoload.php');

$apiInstance = new FastComments\Client\Api\ModerationApi(
    new GuzzleHttp\Client()
);
$sso = 'sso_example'; // string - SSO payload authenticating the moderator

try {
    $result = $apiInstance->getCount(null, null, null, null, null, $sso);
    print_r($result);
} catch (Exception $e) {
    echo 'Exception when calling ModerationApi->getCount: ', $e->getMessage(), PHP_EOL;
}
```