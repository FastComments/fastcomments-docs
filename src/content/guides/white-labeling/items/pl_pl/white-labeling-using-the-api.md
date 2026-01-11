Aby użyć API do utworzenia tenant w modelu white-label, musimy wykonać następujące czynności:

1. Wywołaj the [Tenants API](/guide-api.html#tenant-structure), aby utworzyć tenant.
2. Wywołaj the [TenantPackages API](/guide-api.html#tenant-package-structure), aby utworzyć jeden lub więcej pakietów dla tenanta.
3. Wywołaj the [Tenants API](/guide-api.html#tenant-patch), aby określić, który pakiet jest aktywny dla tenanta.
4. Wywołaj the [TenantUsers API](/guide-api.html#tenant-user-structure), aby dodać użytkowników-administratorów do tenanta.
5. Wywołaj the [Moderators API](/guide-api.html#moderator-structure), aby dodać i zaprosić moderatorów do tenanta.
6. Opcjonalnie, [Skonfiguruj SSO](/guide-customizations-and-configuration.html#sso).

Wszystko to można wykonać w okresie próbnym.