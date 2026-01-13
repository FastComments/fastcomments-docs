API'yi beyaz etiketli bir tenant oluşturmak için kullanmak istiyorsak, aşağıdakileri yapmalıyız:

1. [Tenants API](/guide-api.html#tenant-structure) çağırarak tenant oluşturun.
2. [TenantPackages API](/guide-api.html#tenant-package-structure) çağırarak tenant için bir veya daha fazla paket oluşturun.
3. [Tenants API](/guide-api.html#tenant-patch) çağırarak tenant üzerinde hangi paketin aktif olduğunu tanımlayın.
4. [TenantUsers API](/guide-api.html#tenant-user-structure) çağırarak tenant'a yönetici kullanıcılar ekleyin.
5. [Moderators API](/guide-api.html#moderator-structure) çağırarak tenant'a moderatörler ekleyin ve davet edin.
6. İsteğe bağlı olarak, [SSO Kurulumu](/guide-customizations-and-configuration.html#sso).

Bunların tümü deneme süresi içinde yapılabilir.