---
Ovo je verzija "predugo, nisam čitao" Drupal uputstava.

1. Instalirajte modul pomoću `composer require drupal/fcom`, ili ga postavite u `modules/custom/fastcomments/`.
2. Omogućite ga pomoću `drush en fastcomments`, ili putem administratorskog interfejsa na `/admin/modules`.
3. Idite na `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Unesite svoj Tenant ID i API Secret iz [Podešavanja > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Dodajte polje `FastComments` u bilo koji tip sadržaja putem `Structure > Content types > [type] > Manage fields`.

Modul je objavljen na [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---