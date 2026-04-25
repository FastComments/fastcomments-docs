---
זו הגרסה המקוצרת (TL;DR) של ההוראות ל‑Drupal.

1. התקן את המודול עם `composer require drupal/fcom`, או הנח אותו ב־`modules/custom/fastcomments/`.
2. הפעל אותו באמצעות `drush en fastcomments`, או דרך ממשק הניהול ב־`/admin/modules`.
3. גש אל `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. הזן את ה‑Tenant ID וה‑API Secret מתוך [הגדרות > API/SSO](https://fastcomments.com/auth/my-account/api) ([EU](https://eu.fastcomments.com/auth/my-account/api)).
5. הוסף את השדה `FastComments` לכל סוג תוכן דרך `Structure > Content types > [type] > Manage fields`.

המודול פורסם ב־[drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---