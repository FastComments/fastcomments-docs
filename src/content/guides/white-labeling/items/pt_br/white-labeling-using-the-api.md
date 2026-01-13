---
Para usar a API para criar um tenant com marca branca, devemos fazer o seguinte:

1. Chame a [Tenants API](/guide-api.html#tenant-structure) para criar o tenant.
2. Chame a [TenantPackages API](/guide-api.html#tenant-package-structure) para criar um ou mais pacotes para o tenant.
3. Chame a [Tenants API](/guide-api.html#tenant-patch) para definir qual pacote está ativo no tenant.
4. Chame a [TenantUsers API](/guide-api.html#tenant-user-structure) para adicionar usuários administradores ao tenant.
5. Chame a [Moderators API](/guide-api.html#moderator-structure) para adicionar e convidar moderadores para o tenant.
6. Opcionalmente, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Isto tudo pode ser feito durante o período de avaliação.

---