Da biste koristili API za kreiranje white-label tenant-a, morate uraditi sledeće:

1. Pozovite [Tenants API](/guide-api.html#tenant-structure) da kreirate tenant.
2. Pozovite [TenantPackages API](/guide-api.html#tenant-package-structure) da kreirate jedan ili više paketa za tenant.
3. Pozovite [Tenants API](/guide-api.html#tenant-patch) da definišete koji paket je aktivan na tenantu.
4. Pozovite [TenantUsers API](/guide-api.html#tenant-user-structure) da dodate administratorske korisnike u tenant.
5. Pozovite [Moderators API](/guide-api.html#moderator-structure) da dodate i pozovete moderatore u tenant.
6. Opcionalno, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Sve ovo može da se uradi tokom probnog perioda.