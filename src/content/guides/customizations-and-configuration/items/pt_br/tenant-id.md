[related-parameter-start name = 'tenantId'; type = 'string'; related-parameter-end]

Você pode notar que o widget de comentários pode ser usado com um Tenant ID de "demo", por exemplo:

[code-example-start config = {tenantId: 'demo'}; linesToHighlight = [5]; title = 'Demo Tenant ID'; code-example-end]

Isto serve apenas para experimentar e brincar com o widget de comentários. Em produção, você passaria seu Tenant ID, assim:

[code-example-start config = {tenantId: '{{{ExampleTenantId}}}'}; linesToHighlight = [5]; title = 'Defining Your Tenant ID'; code-example-end]

Seu Tenant ID já pode ser encontrado aplicado no widget de comentários <a href="https://fastcomments.com/auth/my-account/get-acct-code" target="_blank">snippet de código na sua conta</a>.

Você também pode encontrar seu Tenant ID e gerenciar suas chaves de API [na página de credenciais da API](https://fastcomments.com/auth/my-account/api-secret).

A partir deste ponto, se você estiver logado no FastComments, os exemplos de código usarão seu Tenant ID real (se você estiver logado em https://fastcomments.com).

---