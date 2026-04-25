---
Questa è la versione "too long; didn't read" (TL;DR) delle istruzioni per Drupal.

1. Installa il modulo con `composer require drupal/fcom`, oppure mettilo in `modules/custom/fastcomments/`.
2. Attivalo con `drush en fastcomments`, oppure dall'interfaccia di amministrazione su `/admin/modules`.
3. Vai a `Administration > Configuration > Content > FastComments` (`/admin/config/content/fastcomments`).
4. Inserisci il tuo Tenant ID e API Secret da [Impostazioni > API/SSO](https://fastcomments.com/auth/my-account/api) ([UE](https://eu.fastcomments.com/auth/my-account/api)).
5. Aggiungi il campo `FastComments` a qualsiasi tipo di contenuto tramite `Structure > Content types > [type] > Manage fields`.

Il modulo è pubblicato su [drupal.org/project/fcom](https://www.drupal.org/project/fcom).

---