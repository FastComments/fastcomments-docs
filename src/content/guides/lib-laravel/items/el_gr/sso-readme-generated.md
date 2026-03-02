Ενεργοποιήστε το SSO στο αρχείο `.env`:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Το API key απαιτείται για το ασφαλές SSO — χρησιμοποιείται για την υπογραφή του payload του SSO.

### Χαρτογράφηση με βάση το config

Στο `config/fastcomments.php`, αντιστοιχίστε τα πεδία του FastComments με τα attributes του μοντέλου User σας:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // υποστηρίζεται σημειογραφία με τελείες (dot notation)
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Χαρτογράφηση βάσει Interface

Για περισσότερο έλεγχο, υλοποιήστε το interface `MapsToFastCommentsUser` στο μοντέλο User σας:

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

Όταν υλοποιείται το interface, έχει προτεραιότητα έναντι της χαρτογράφησης βάσει config.

### SSO στο Blade

Όταν το SSO είναι ενεργοποιημένο, το component `<x-fastcomments />` εισάγει αυτόματα δεδομένα SSO για τον πιστοποιημένο χρήστη.