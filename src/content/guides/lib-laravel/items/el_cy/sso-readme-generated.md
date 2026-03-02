Ενεργοποιήστε το SSO στο `.env` σας:

```env
FASTCOMMENTS_API_KEY=your-api-key
FASTCOMMENTS_SSO_ENABLED=true
FASTCOMMENTS_SSO_MODE=secure
```

Το API key είναι απαραίτητο για ασφαλές SSO — χρησιμοποιείται για την υπογραφή του SSO payload.

### Config-Based Mapping

Στο `config/fastcomments.php`, αντιστοιχίστε τα πεδία του FastComments με τα χαρακτηριστικά του μοντέλου User σας:

```php
'sso' => [
    'enabled' => true,
    'mode' => 'secure',
    'user_map' => [
        'id' => 'id',
        'email' => 'email',
        'username' => 'name',
        'avatar' => 'profile.avatar_url', // υποστηρίζεται σημειογραφία με τελείες
    ],
    'is_admin' => fn ($user) => $user->hasRole('admin'),
    'is_moderator' => fn ($user) => $user->hasRole('moderator'),
],
```

### Interface-Based Mapping

Για μεγαλύτερο έλεγχο, υλοποιήστε το interface `MapsToFastCommentsUser` στο μοντέλο User σας:

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

Όταν το interface υλοποιείται, έχει προτεραιότητα σε σχέση με την αντιστοίχιση που βασίζεται στο config.

### SSO in Blade

Όταν το SSO είναι ενεργοποιημένο, το component `<x-fastcomments />` εισάγει αυτόματα τα δεδομένα SSO για τον αυθεντικοποιημένο χρήστη.