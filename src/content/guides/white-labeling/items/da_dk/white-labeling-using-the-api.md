For at bruge API'en til at oprette en white-label tenant, skal vi gøre følgende:

1. Kald [Tenants API](/guide-api.html#tenant-structure) for at oprette tenanten.
2. Kald [TenantPackages API](/guide-api.html#tenant-package-structure) for at oprette én eller flere pakker til tenanten.
3. Kald [Tenants API](/guide-api.html#tenant-patch) for at definere, hvilken pakke der er aktiv på tenanten.
4. Kald [TenantUsers API](/guide-api.html#tenant-user-structure) for at tilføje administratorbrugere til tenanten.
5. Kald [Moderators API](/guide-api.html#moderator-structure) for at tilføje og invitere moderatorer til tenanten.
6. Valgfrit, [Opsæt SSO](/guide-customizations-and-configuration.html#sso).

Alt dette kan gøres inden for prøveperioden.