Για να χρησιμοποιήσουμε το API για τη δημιουργία ενός white labeled tenant, πρέπει να κάνουμε τα εξής:

1. Να καλέσουμε το [Tenants API](/guide-api.html#tenant-structure) για να δημιουργήσουμε τον tenant.
2. Να καλέσουμε το [TenantPackages API](/guide-api.html#tenant-package-structure) για να δημιουργήσουμε ένα ή περισσότερα πακέτα για τον tenant.
3. Να καλέσουμε το [Tenants API](/guide-api.html#tenant-patch) για να ορίσουμε ποιο πακέτο είναι ενεργό στον tenant.
4. Να καλέσουμε το [TenantUsers API](/guide-api.html#tenant-user-structure) για να προσθέσουμε χρήστες διαχειριστές στον tenant.
5. Να καλέσουμε το [Moderators API](/guide-api.html#moderator-structure) για να προσθέσουμε και να προσκαλέσουμε moderators στον tenant.
6. Προαιρετικά, να [ρυθμίσουμε το SSO](/guide-customizations-and-configuration.html#sso).

Όλα αυτά μπορούν να γίνουν εντός της δοκιμαστικής περιόδου.