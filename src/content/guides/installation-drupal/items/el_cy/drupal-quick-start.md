Αυτή είναι η έκδοση «too long did not read» των οδηγιών για το Drupal.

1. Εγκαταστήστε το module με `composer require drupal/fcom`, ή τοποθετήστε το στο `modules/custom/fastcomments/`.
2. Ενεργοποιήστε το με `drush en fastcomments`, ή από το διαχειριστικό περιβάλλον στο `/admin/modules`.
3. Πηγαίνετε στο `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Εισάγετε το Tenant ID και το API Secret σας από [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Προσθέστε το πεδίο `FastComments` σε οποιονδήποτε τύπο περιεχομένου μέσω `Structure > Content types > [type] > Manage fields`.

Η μονάδα είναι δημοσιευμένη στο [drupal.org/project/fcom](https://www.drupal.org/project/fcom).