---
За да използваме API-то за създаване на брандиран (white-label) наемател, трябва да направим следното:

1. Извикайте [Tenants API](/guide-api.html#tenant-structure) за да създадете наемателя.
2. Извикайте [TenantPackages API](/guide-api.html#tenant-package-structure) за да създадете един или повече пакети за наемателя.
3. Извикайте [Tenants API](/guide-api.html#tenant-patch) за да зададете кой пакет е активен за наемателя.
4. Извикайте [TenantUsers API](/guide-api.html#tenant-user-structure) за да добавите администраторски потребители към наемателя.
5. Извикайте [Moderators API](/guide-api.html#moderator-structure) за да добавите и поканите модератори към наемателя.
6. По избор, [Настройте SSO](/guide-customizations-and-configuration.html#sso).

Всичко това може да бъде направено в рамките на пробния период.

---