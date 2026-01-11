APIを使用してホワイトラベルのテナントを作成するには、以下を行う必要があります：

1. テナントを作成するために [Tenants API](/guide-api.html#tenant-structure) を呼び出します。
2. テナント用に1つ以上のパッケージを作成するために [TenantPackages API](/guide-api.html#tenant-package-structure) を呼び出します。
3. テナント上でどのパッケージがアクティブかを定義するために [Tenants API](/guide-api.html#tenant-patch) を呼び出します。
4. テナントに管理者ユーザーを追加するために [TenantUsers API](/guide-api.html#tenant-user-structure) を呼び出します。
5. テナントにモデレーターを追加および招待するために [Moderators API](/guide-api.html#moderator-structure) を呼び出します。
6. 任意で、[Setup SSO](/guide-customizations-and-configuration.html#sso)。

これらはすべてトライアル期間内に行うことができます。