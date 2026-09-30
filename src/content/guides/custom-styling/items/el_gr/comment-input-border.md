Το περίγραμμα γύρω από το κουτί σχολίων αποτελείται από το δικό του περίγραμμα της περιοχής κειμένου συν μερικές λεπτές γραμμές που σχεδιάζει το widget γύρω του. Για να αλλάξετε το χρώμα ή την στρογγυλότητα των γωνιών, ορίστε αυτές τις μεταβλητές CSS αντί να στυλιζάτε άμεσα το `textarea`. Οι μεταβλητές επαναστυλιζούν όλο το περίγραμμα ταυτόχρονα, ώστε οι γωνίες και τα χρώματα να ταιριάζουν πάντα.

| Μεταβλητή | Τι αλλάζει | Προεπιλογή |
|---|---|---|
| `--fc-input-border-color` | Χρώμα περιγράμματος | `#bfbfbf` |
| `--fc-input-border-color-focus` | Χρώμα περιγράμματος ενώ ο χρήστης πληκτρολογεί | `#555` |
| `--fc-input-border-radius` | Στρογγυλότητα των στρογγυλεμένων γωνιών | `11px` |
| `--fc-input-border-start-start-radius` | Στρογγυλότητα της τετράγωνης γωνίας πάνω‑αριστερά (πάνω‑δεξιά σε γλώσσες από δεξιά προς αριστερά) | `0` |

Προσθέστε το CSS στο πεδίο **Custom CSS** στη [Widget Customization page](https://fastcomments.com/auth/my-account/customize-widget), ή περάστε το με την επιλογή `customCSS`. Χρειάζεται μόνο να ορίσετε τις μεταβλητές που θέλετε να αλλάξετε.

Μπορείτε επίσης να χρησιμοποιήσετε τον βοηθό **Comment box border** ακριβώς κάτω από το πεδίο Custom CSS, ο οποίος δημιουργεί αυτόματα αυτό το CSS για εσάς.

## Αλλαγή Χρώματος Περιγράμματος

[inline-code-attrs-start title = 'Χρώμα Περιγράμματος'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
[inline-code-end]

## Αλλαγή Χρώματος Περιγράμματος Κατά την Πληκτρολόγηση

[inline-code-attrs-start title = 'Χρώμα Περιγράμματος Κατά την Πληκτρολόγηση'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color-focus: #2563eb;
}
[inline-code-end]

## Στρογγυλοποίηση Όλων των Τεσσάρων Γωνιών

Από προεπιλογή η πάνω‑αριστερή γωνία είναι τετράγωνη. Ορίστε και τις δύο μεταβλητές ακτίνας για να στρογγυλοποιήσετε όλες τις γωνίες με το ίδιο μέγεθος:

[inline-code-attrs-start title = 'Στρογγυλοποίηση Όλων των Τεσσάρων Γωνιών'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 12px;
    --fc-input-border-start-start-radius: 12px;
}
[inline-code-end]

## Τετράγωνες Γωνίες

[inline-code-attrs-start title = 'Τετράγωνες Γωνίες'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-radius: 0;
}
[inline-code-end]

## Ταιριάξτε το Brand Σας

[inline-code-attrs-start title = 'Χρώματα Brand και Γωνίες'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #c7d2fe;
    --fc-input-border-color-focus: #4f46e5;
    --fc-input-border-radius: 8px;
    --fc-input-border-start-start-radius: 8px;
}
[inline-code-end]

## Διαφορετικά Χρώματα σε Dark Mode

Όταν το widget βρίσκεται σε dark mode έχει την κλάση `dark`, οπότε μπορείτε να ορίσετε διαφορετικές τιμές για dark mode:

[inline-code-attrs-start title = 'Χρώματα Περιγράμματος σε Dark Mode'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
:root {
    --fc-input-border-color: #d1d5db;
}
.dark {
    --fc-input-border-color: #444;
    --fc-input-border-color-focus: #aaa;
}
[inline-code-end]

## Γιατί να Μην Στυλιζάτε Άμεσα το Text Area;

Το widget σχεδιάζει μέρος του περιγράμματος του κουτιού σχολίων γύρω από την περιοχή κειμένου. Αν ορίσετε `border-color` ή `border-radius` μόνο στο `textarea`, αυτές οι γραμμές διατηρούν το προεπιλεγμένο στυλ και το περίγραμμα φαίνεται ασύμφωνο, π.χ. μια τετράγωνη γραμμή που διασχίζει μια στρογγυλεμένη γωνία. Οι παραπάνω μεταβλητές αλλάζουν και τα δύο ταυτόχρονα.