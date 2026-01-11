Um die API zu verwenden, um einen White-Label tenant zu erstellen, müssen wir Folgendes tun:

1. Rufen Sie die [Tenants API](/guide-api.html#tenant-structure) auf, um den tenant zu erstellen.
2. Rufen Sie die [TenantPackages API](/guide-api.html#tenant-package-structure) auf, um ein oder mehrere Pakete für den tenant zu erstellen.
3. Rufen Sie die [Tenants API](/guide-api.html#tenant-patch) auf, um festzulegen, welches Paket auf dem tenant aktiv ist.
4. Rufen Sie die [TenantUsers API](/guide-api.html#tenant-user-structure) auf, um Administratoren zum tenant hinzuzufügen.
5. Rufen Sie die [Moderators API](/guide-api.html#moderator-structure) auf, um Moderatoren zum tenant hinzuzufügen und einzuladen.
6. Optional, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Das alles kann während der Testphase erledigt werden.