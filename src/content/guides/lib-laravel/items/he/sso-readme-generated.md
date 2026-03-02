---
הפעל SSO בקובץ `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

מפתח ה-API נדרש עבור SSO מאובטח — הוא משמש לחתימת מטען ה-SSO.

### מיפוי מבוסס קונפיגורציה

ב-`config/fastcomments.php`, מיפו את שדות FastComments לשדות במודל המשתמש שלכם:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // נוטציית נקודות נתמכת
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### מיפוי מבוסס ממשק

לקבלת שליטה רבה יותר, יישמו את הממשק `MapsToFastCommentsUser` על מודל המשתמש שלכם:

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

כאשר הממשק מיושם, הוא מקבל עדיפות על פני המיפוי המבוסס קונפיגורציה.

### SSO ב-Blade

כש-SSO מופעל, הרכיב `<x-fastcomments />` מזריק אוטומטית נתוני SSO עבור המשתמש שאומת.
---