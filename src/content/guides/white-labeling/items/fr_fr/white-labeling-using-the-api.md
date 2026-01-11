---
Pour utiliser l'API afin de créer un tenant en marque blanche, nous devons effectuer les opérations suivantes :

1. Appeler la [Tenants API](/guide-api.html#tenant-structure) pour créer le tenant.
2. Appeler la [TenantPackages API](/guide-api.html#tenant-package-structure) pour créer un ou plusieurs packages pour le tenant.
3. Appeler la [Tenants API](/guide-api.html#tenant-patch) pour définir quel package est actif sur le tenant.
4. Appeler la [TenantUsers API](/guide-api.html#tenant-user-structure) pour ajouter des utilisateurs administrateurs au tenant.
5. Appeler la [Moderators API](/guide-api.html#moderator-structure) pour ajouter et inviter des modérateurs au tenant.
6. Optionnellement, [Configurer le SSO](/guide-customizations-and-configuration.html#sso).

Tout cela peut être réalisé pendant la période d'essai.

---