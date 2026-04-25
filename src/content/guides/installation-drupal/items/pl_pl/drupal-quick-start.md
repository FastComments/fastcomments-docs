---
To jest wersja „TL;DR” instrukcji dla Drupala.

1. Zainstaluj moduł za pomocą `composer require drupal/fcom`, lub umieść go w `modules/custom/fastcomments/`.
2. Włącz go za pomocą `drush en fastcomments`, lub z poziomu interfejsu administracyjnego pod adresem `/admin/modules`.
3. Przejdź do `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Wprowadź swój Tenant ID i API Secret z [Ustawienia > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
5. Dodaj pole `FastComments` do dowolnego typu treści za pomocą `Structure > Content types > [type] > Manage fields`.

Moduł jest opublikowany na [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---