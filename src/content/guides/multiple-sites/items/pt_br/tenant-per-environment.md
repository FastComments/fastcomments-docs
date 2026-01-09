É comum ter um sub-tenant por ambiente de teste ou desenvolvimento com o FastComments. Cada tenant teria sua própria configuração, dados e chaves de API. Configuração, dados e usuários não podem ser compartilhados entre tenants.
Tudo é isolado. No entanto, superadministradores do tenant principal podem se passar por usuários em tenants filhos.

There are two approaches:

- The main tenant is for production, and sub-tenants are for test environments.
- The main tenant is simply for billing, and each sub-tenant is for prod, test, and so on.

A primeira geralmente é mais fácil para os usuários compreenderem, mas isso pode depender da sua organização.

Tenants podem ser criados [aqui](https://eu.fastcomments.com/auth/my-account/tenants) se você tiver o pacote. É também onde superadministradores iriam se passar por usuários. Tenants também podem ser criados via API para configurações mais personalizadas/automatizadas.

Independentemente da abordagem escolhida, você terá que adicionar os moderadores e usuários que desejam ver os dados de produção no tenant "prod". Então, por exemplo, se você quiser seguir a opção B e usar o tenant pai apenas para faturamento, e tiver um sub-tenant para "prod", você vai querer adicionar o tenant, alternar para o novo tenant e adicionar seus usuários administradores e moderadores para o sub-tenant. 

Por fim, para esclarecer, a página Moderar Comentários ficará vazia com a opção B para o tenant principal.