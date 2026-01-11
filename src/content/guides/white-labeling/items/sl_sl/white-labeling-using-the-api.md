---
Za uporabo API-ja za ustvarjanje white-label najemnika moramo narediti naslednje:

1. Pokličite [Tenants API](/guide-api.html#tenant-structure) za ustvarjanje najemnika.
2. Pokličite [TenantPackages API](/guide-api.html#tenant-package-structure) za ustvarjanje enega ali več paketov za najemnika.
3. Pokličite [Tenants API](/guide-api.html#tenant-patch) da določite, kateri paket je aktiven na najemniku.
4. Pokličite [TenantUsers API](/guide-api.html#tenant-user-structure) za dodajanje skrbniških uporabnikov v najemnika.
5. Pokličite [Moderators API](/guide-api.html#moderator-structure) za dodajanje in povabilo moderatorjev v najemnika.
6. Po želji, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Vse to je mogoče narediti v poskusnem obdobju.

---