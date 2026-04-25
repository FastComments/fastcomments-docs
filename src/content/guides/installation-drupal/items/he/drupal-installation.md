---
התוסף FastComments ל-Drupal מחליף את מערכת התגובות המובנית של Drupal במערכת תגובות מהירה ובזמן אמת. התוסף הוא [פורסם ב-drupal.org](https://www.drupal.org/project/fcom) ועובד עם Drupal 10 ו-11.

There are two ways to install it.

## התקנה עם Composer

```
composer require drupal/fcom
drush en fastcomments
```

## התקנה ידנית

הורד את התוסף מ-[drupal.org/project/fcom](https://www.drupal.org/project/fcom) והנח אותו בתיקיית האתר שלך `modules/custom/fastcomments/`. לאחר מכן הפעל אותו עם `drush en fastcomments`, או מה-UI הניהולי ב-`Extend` (`/admin/modules`).

<sup>שימו לב!</sup> התוסף תלוי רק ב-core של Drupal (`user` ו-`field`). אין מודולים או ספריות Drupal נוספות הנדרשות.

Once the module is enabled, head to the `Configuration` section to set up your Tenant ID and API Secret.
---