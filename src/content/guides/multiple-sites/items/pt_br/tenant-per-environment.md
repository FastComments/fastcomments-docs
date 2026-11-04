É comum ter um sub tenant por ambiente de teste ou desenvolvimento com o FastComments. Cada tenant teria sua própria configuração, dados e chaves de API. Configuração, dados e usuários não podem ser compartilhados entre tenants.
Tudo é isolado. No entanto, super admins do tenant pai podem personificar usuários em tenants filhos.

Existem duas abordagens:

- O tenant principal é para produção, e os sub-tenants são para ambientes de teste.
- O tenant principal é apenas para cobrança, e cada sub-tenant é para prod, teste e assim por diante.

A primeira é geralmente mais fácil de entender para os usuários, mas isso pode depender da sua organização.

Tenants podem ser criados [aqui](https://eu.fastcomments.com/auth/my-account/tenants) se você tiver o pacote. É também onde super admins
personificam usuários. Tenants também podem ser criados via API para configurações mais personalizadas/automatizadas.

Independentemente da abordagem adotada, você terá que adicionar os moderadores e usuários que desejam ver os dados de produção no tenant "prod". Então, por exemplo, se você quiser
optar pela opção B e ter o tenant pai para cobrança, e ter um sub tenant para "prod", você vai querer adicionar o tenant, alternar para o novo tenant e adicionar seus
usuários admin e moderadores para o sub-tenant.

Por fim, para esclarecer, a página Moderate Comments estará vazia com a opção B para o tenant pai.