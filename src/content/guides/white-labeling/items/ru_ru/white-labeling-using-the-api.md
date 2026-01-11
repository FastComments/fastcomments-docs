---
Чтобы использовать API для создания тенанта с белой маркировкой, необходимо выполнить следующее:

1. Вызвать [Tenants API](/guide-api.html#tenant-structure) чтобы создать тенант.
2. Вызвать [TenantPackages API](/guide-api.html#tenant-package-structure) чтобы создать один или несколько пакетов для тенанта.
3. Вызвать [Tenants API](/guide-api.html#tenant-patch) чтобы определить, какой пакет активен у тенанта.
4. Вызвать [TenantUsers API](/guide-api.html#tenant-user-structure) чтобы добавить администраторов в тенант.
5. Вызвать [Moderators API](/guide-api.html#moderator-structure) чтобы добавить и пригласить модераторов в тенант.
6. Опционально, [Настройте SSO](/guide-customizations-and-configuration.html#sso).

Всё это можно сделать в течение пробного периода.

---