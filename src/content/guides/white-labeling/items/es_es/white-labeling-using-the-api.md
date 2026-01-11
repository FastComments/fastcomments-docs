Para usar la API para crear un tenant de marca blanca, debemos hacer lo siguiente:

1. Llamar al [Tenants API](/guide-api.html#tenant-structure) para crear el tenant.
2. Llamar al [TenantPackages API](/guide-api.html#tenant-package-structure) para crear uno o más paquetes para el tenant.
3. Llamar al [Tenants API](/guide-api.html#tenant-patch) para definir qué paquete está activo en el tenant.
4. Llamar al [TenantUsers API](/guide-api.html#tenant-user-structure) para añadir usuarios administradores al tenant.
5. Llamar al [Moderators API](/guide-api.html#moderator-structure) para agregar e invitar moderadores al tenant.
6. Opcionalmente, [Setup SSO](/guide-customizations-and-configuration.html#sso).

Esto se puede hacer todo dentro del período de prueba.