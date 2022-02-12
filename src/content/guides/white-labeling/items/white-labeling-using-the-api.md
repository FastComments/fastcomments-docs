To use the API to create a white labeled tenant, we must do the following:

1. Call the [Tenants API](/guide-api.html#tenant-structure) to create the tenant.
2. Call the [TenantPackages API](/guide-api.html#tenant-package-structure) to create one or more packages for the tenant.
3. Call the [Tenants API](/guide-api.html#tenant-patch) to define which package is active on the tenant.
4. Call the [TenantUsers API](/guide-api.html#tenant-user-structure) to add administrator users to the tenant.
5. Call the [Moderators API](/guide-api.html#moderator-structure) to add and invite moderators to the tenant.
6. Optionally, [Setup SSO](/guide-customizations-and-configuration.html#sso).

This can all be done within the trial period.
