---
Да бисмо користили API да креирамо tenant са прилагођеним брендом, морамо урадити следеће:

1. Позовите [Tenants API](/guide-api.html#tenant-structure) да бисте креирали tenant.
2. Позовите [TenantPackages API](/guide-api.html#tenant-package-structure) да бисте креирали један или више пакета за tenant.
3. Позовите [Tenants API](/guide-api.html#tenant-patch) да бисте дефинисали који пакет је активан на tenant-у.
4. Позовите [TenantUsers API](/guide-api.html#tenant-user-structure) да бисте додали администраторске кориснике у tenant.
5. Позовите [Moderators API](/guide-api.html#moderator-structure) да бисте додали и позвали модераторе у tenant.
6. Опционално, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Све ово се може урадити током пробног периода.

---