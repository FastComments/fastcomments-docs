[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Vous pouvez constater que le widget de commentaires peut être utilisé avec un Tenant ID de "demo", par exemple:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Ceci sert uniquement à tester et à expérimenter avec le widget de commentaires. En production, vous devriez fournir votre Tenant ID, comme ceci:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Votre Tenant ID se trouve déjà appliqué dans le <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">extrait de code du widget de commentaires dans votre compte</a>.

Vous pouvez également trouver votre Tenant ID et gérer vos clés API [sur la page des identifiants API](https://fastcomments.com/auth/my-account/api-secret).

À partir de maintenant, si vous êtes connecté à FastComments, les exemples de code utiliseront votre vrai Tenant ID (si vous êtes connecté sur https://fastcomments.com).

---