Om de API te gebruiken om een white-label tenant aan te maken, moeten we het volgende doen:

1. Roep de [Tenants API](/guide-api.html#tenant-structure) aan om de tenant te maken.
2. Roep de [TenantPackages API](/guide-api.html#tenant-package-structure) aan om één of meer pakketten voor de tenant te maken.
3. Roep de [Tenants API](/guide-api.html#tenant-patch) aan om te bepalen welk pakket actief is op de tenant.
4. Roep de [TenantUsers API](/guide-api.html#tenant-user-structure) aan om beheerders aan de tenant toe te voegen.
5. Roep de [Moderators API](/guide-api.html#moderator-structure) aan om moderators aan de tenant toe te voegen en uit te nodigen.
6. Optioneel: [SSO instellen](/guide-customizations-and-configuration.html#sso).

Dit kan allemaal binnen de proefperiode worden gedaan.