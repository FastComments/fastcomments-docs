Da bismo koristili API za kreiranje white-label tenanta, moramo slijediti sljedeće:

1. Pozovite the [Tenants API](/guide-api.html#tenant-structure) da kreirate tenanta.
2. Pozovite the [TenantPackages API](/guide-api.html#tenant-package-structure) da kreirate jedan ili više paketa za tenanta.
3. Pozovite the [Tenants API](/guide-api.html#tenant-patch) da odredite koji paket je aktivan na tenantu.
4. Pozovite the [TenantUsers API](/guide-api.html#tenant-user-structure) da dodate administratorske korisnike na tenanta.
5. Pozovite the [Moderators API](/guide-api.html#moderator-structure) da dodate i pozovete moderatore na tenanta.
6. Opcionalno, [Podesite SSO](/guide-customizations-and-configuration.html#sso).

Sve ovo se može uraditi tokom probnog perioda.

---