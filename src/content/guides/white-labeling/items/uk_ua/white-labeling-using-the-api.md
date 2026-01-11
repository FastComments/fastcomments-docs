Щоб скористатися API для створення тенанта з білою міткою, потрібно виконати наступне:

1. Викличте [Tenants API](/guide-api.html#tenant-structure), щоб створити тенанта.
2. Викличте [TenantPackages API](/guide-api.html#tenant-package-structure), щоб створити один або кілька пакетів для тенанта.
3. Викличте [Tenants API](/guide-api.html#tenant-patch), щоб визначити, який пакет активний у тенанті.
4. Викличте [TenantUsers API](/guide-api.html#tenant-user-structure), щоб додати адміністративних користувачів до тенанта.
5. Викличте [Moderators API](/guide-api.html#moderator-structure), щоб додати та запросити модераторів до тенанта.
6. За бажанням, [Налаштуйте SSO](/guide-customizations-and-configuration.html#sso).

Усе це можна зробити протягом пробного періоду.