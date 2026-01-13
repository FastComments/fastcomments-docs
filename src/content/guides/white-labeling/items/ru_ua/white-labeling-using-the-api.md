---
Чтобы использовать API для создания тенанта с белой маркировкой, необходимо выполнить следующие действия:

1. Вызвать the [Tenants API](/guide-api.html#tenant-structure) для создания тенанта.
2. Вызвать the [TenantPackages API](/guide-api.html#tenant-package-structure) для создания одного или нескольких пакетов для тенанта.
3. Вызвать the [Tenants API](/guide-api.html#tenant-patch) для определения, какой пакет активен у тенанта.
4. Вызвать the [TenantUsers API](/guide-api.html#tenant-user-structure) чтобы добавить пользователей-администраторов в тенант.
5. Вызвать the [Moderators API](/guide-api.html#moderator-structure) чтобы добавить и пригласить модераторов в тенант.
6. Опционально, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Всё это можно сделать в течение пробного периода.

---