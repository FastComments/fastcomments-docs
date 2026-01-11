---
要使用 API 來建立白標租戶，我們必須執行下列步驟：

1. 呼叫 [Tenants API](/guide-api.html#tenant-structure) 來建立租戶。
2. 呼叫 [TenantPackages API](/guide-api.html#tenant-package-structure) 為租戶建立一個或多個套件。
3. 呼叫 [Tenants API](/guide-api.html#tenant-patch) 以定義租戶啟用哪一個套件。
4. 呼叫 [TenantUsers API](/guide-api.html#tenant-user-structure) 將管理員使用者新增到租戶。
5. 呼叫 [Moderators API](/guide-api.html#moderator-structure) 向租戶新增並邀請 moderators。
6. 可選地，[Setup SSO](/guide-customizations-and-configuration.html#sso)。

所有這些都可以在試用期間完成。

---