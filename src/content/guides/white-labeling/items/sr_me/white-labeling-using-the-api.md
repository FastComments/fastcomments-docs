Da bismo koristili API za kreiranje white-label tenanta, moramo uraditi sljedeće:

1. Pozovite [Tenants API](/guide-api.html#tenant-structure) da biste kreirali tenant.
2. Pozovite [TenantPackages API](/guide-api.html#tenant-package-structure) da biste kreirali jedan ili više paketa za tenant.
3. Pozovite [Tenants API](/guide-api.html#tenant-patch) da biste odredili koji je paket aktivan na tenantu.
4. Pozovite [TenantUsers API](/guide-api.html#tenant-user-structure) da biste dodali administratorske korisnike u tenant.
5. Pozovite [Moderators API](/guide-api.html#moderator-structure) da biste dodali i pozvali moderatore u tenant.
6. Opcionalno, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Sve ovo se može uraditi tokom probnog perioda.

---