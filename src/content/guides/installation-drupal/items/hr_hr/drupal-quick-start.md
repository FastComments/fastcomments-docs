Ovo je "predugo, nisam čitao" verzija uputa za Drupal.

1. Instalirajte modul pomoću `composer require drupal/fcom`, ili ga stavite u `modules/custom/fastcomments/`.
2. Omogućite ga pomoću `drush en fastcomments`, ili iz administratorskog sučelja na `/admin/modules`.
3. Idite na `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Unesite svoj Tenant ID i API Secret iz [Settings > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Dodajte polje `FastComments` bilo kojem tipu sadržaja putem `Structure > Content types > [type] > Manage fields`.

Modul je objavljen na [drupal.org/project/fcom](https://www.drupal.org/project/fcom).