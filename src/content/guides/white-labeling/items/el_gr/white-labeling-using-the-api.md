---
Για να χρησιμοποιήσετε το API για να δημιουργήσετε έναν tenant με white-label, πρέπει να κάνετε τα εξής:

1. Καλέστε το [Tenants API](/guide-api.html#tenant-structure) για να δημιουργήσετε τον tenant.
2. Καλέστε το [TenantPackages API](/guide-api.html#tenant-package-structure) για να δημιουργήσετε ένα ή περισσότερα πακέτα για τον tenant.
3. Καλέστε το [Tenants API](/guide-api.html#tenant-patch) για να ορίσετε ποιο πακέτο είναι ενεργό στον tenant.
4. Καλέστε το [TenantUsers API](/guide-api.html#tenant-user-structure) για να προσθέσετε χρήστες-διαχειριστές στον tenant.
5. Καλέστε το [Moderators API](/guide-api.html#moderator-structure) για να προσθέσετε και να προσκαλέσετε moderators στον tenant.
6. Προαιρετικά, [Setup SSO](/guide-customizations-and-configuration.html#sso).

This can all be done within the trial period.

---