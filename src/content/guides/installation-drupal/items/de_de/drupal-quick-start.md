---
Dies ist die "zu lang; nicht gelesen"-Version der Drupal-Anleitung.

1. Installieren Sie das Modul mit `composer require drupal/fcom`, oder legen Sie es in `modules/custom/fastcomments/` ab.
2. Aktivieren Sie es mit `drush en fastcomments`, oder über die Admin-Oberfläche unter `/admin/modules`.
3. Gehen Sie zu `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Geben Sie Ihre Tenant-ID und Ihr API-Secret aus [Einstellungen > API/SSO](https://fastcomments.com/auth/my-account/api) ein ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. Fügen Sie das Feld `FastComments` zu einem beliebigen Inhaltstyp über `Structure > Content types > [type] > Manage fields` hinzu.

Das Modul ist auf [drupal.org/project/fcom](https://www.drupal.org/project/fcom) veröffentlicht.

---