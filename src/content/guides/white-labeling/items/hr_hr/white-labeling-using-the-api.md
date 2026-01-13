---
Za korištenje API-ja za stvaranje white-labeled tenanta, moramo učiniti sljedeće:

1. Pozovite [Tenants API](/guide-api.html#tenant-structure) kako biste stvorili tenanta.
2. Pozovite [TenantPackages API](/guide-api.html#tenant-package-structure) kako biste stvorili jedan ili više paketa za tenanta.
3. Pozovite [Tenants API](/guide-api.html#tenant-patch) kako biste definirali koji je paket aktivan na tenantu.
4. Pozovite [TenantUsers API](/guide-api.html#tenant-user-structure) kako biste dodali administratorske korisnike u tenant.
5. Pozovite [Moderators API](/guide-api.html#moderator-structure) kako biste dodali i pozvali moderatore u tenant.
6. Opcionalno, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Sve se to može obaviti tijekom probnog razdoblja.
---