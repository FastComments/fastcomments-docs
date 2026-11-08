अपनी `.env` फ़ाइल में SSO सक्षम करें:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

सुरक्षित SSO के लिए API key आवश्यक है — इसका उपयोग SSO payload पर हस्ताक्षर करने के लिए किया जाता है।

### Config-आधारित मैपिंग

`config/fastcomments.php` में, FastComments फ़ील्ड्स को अपने User model के attributes से मैप करें:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // डॉट नोटेशन समर्थित है
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-आधारित मैपिंग

अधिक नियंत्रण के लिए, अपने User model पर `MapsToFastCommentsUser` interface लागू करें:

```php
use FastComments\Laravel\SSO\Contracts\MapsToFastCommentsUser;

class User extends Authenticatable implements MapsToFastCommentsUser
{
    public function toFastCommentsUserData(): array
    {
        return [
            'id' => (string) $this->id,
            'email' => $this->email,
            'username' => $this->display_name,
            'avatar' => $this->avatar_url,
            'is_admin' => $this->hasRole('admin'),
        ];
    }
}
```

जब interface लागू किया गया हो, तो यह config-आधारित मैपिंग पर प्राथमिकता लेता है।

### Blade में SSO

जब SSO सक्षम हो, तो `<x-fastcomments />` कंपोनेंट प्रमाणित उपयोगकर्ता के लिए SSO डेटा स्वचालित रूप से इंजेक्ट करता है।