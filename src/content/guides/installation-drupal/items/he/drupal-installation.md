מודול ה-FastComments ל-Drupal מחליף את מערכת התגובות המובנית של Drupal במערכת תגובות מהירה ובזמן אמת. המודול [מפורסם ב-drupal.org](https://www.drupal.org/project/fcom) ועובד עם Drupal 10 ו-11.

ישנן שתי דרכים להתקין אותו.

## התקנה עם Composer

```
composer require drupal/fcom
drush en fastcomments
```

## התקנה ידנית

הורידו את המודול מ-[drupal.org/project/fcom](https://www.drupal.org/project/fcom) ומקמו אותו בתיקיית `modules/custom/fastcomments/` של האתר שלכם. לאחר מכן הפעילו אותו עם `drush en fastcomments`, או מממשק הניהול ב-`Extend` (`/admin/modules`).

<sup>שימו לב!</sup> המודול תלוי רק בליבת Drupal (`user` ו-`field`). אין צורך במודולים או ספריות Drupal אחרים.

לאחר הפעלת המודול, עברו לסעיף `Configuration` כדי להגדיר את ה-Tenant ID ואת ה-API Secret שלכם.