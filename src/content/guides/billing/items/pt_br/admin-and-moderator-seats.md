No Flex, as pessoas que podem administrar ou moderar sua conta são cobradas como assentos mensais. Essa é a parte da fatura do Flex que mais costuma surpreender as pessoas, geralmente porque os assentos foram criados sem que ninguém percebesse.

### O Que Conta Como Um Assento

- **Admins**: usuários na página `Users` da sua conta com qualquer permissão administrativa (faturamento, personalização, análises, moderação e assim por diante). O primeiro admin, normalmente o proprietário da conta, é gratuito; cada admin adicional é um assento.
- **Moderadores**: pessoas na página `Moderators` que aceitaram o convite. Um convite pendente não é cobrado. Um usuário cuja única permissão é a moderação de comentários é cobrado como moderador, não como admin.
- **SSO Admins** e **SSO Moderators**: usuários SSO com os quais seu site faz login com `isAccountOwner`, `isAdminAdmin` ou `isCommentModeratorAdmin` definidos no payload SSO. Eles são cobrados às mesmas tarifas que admins e moderadores regulares. Veja [SSO User Structure](/guide-api.html#sso-user-structure).

Os assentos são deduplicados por endereço de e-mail, então uma pessoa que é tanto admin quanto moderadora, ou que tem tanto um usuário SSO quanto um login no painel com o mesmo e-mail, é cobrada uma única vez, pela tarifa mais alta. Alguém com dois endereços de e-mail diferentes conta como dois assentos.

Em contas cobradas por usuários ativos mensais, um moderador SSO que não fez login durante o mês não é cobrado por esse mês.

### Assentos Que Você Não Adicionou

Dois recursos criam assentos automaticamente:

- **Sincronização de admins SSO.** Quando um usuário SSO faz login com um e-mail que corresponde a um dos usuários do seu painel, as permissões de admin ou moderador desse usuário são copiadas para o usuário SSO. Com o SSO do WordPress, isso significa que todo administrador do WordPress pode se tornar um admin ou moderador do FastComments. Desative isso em `Customize` com a opção `Disable Automatic Admin Migration` na sua regra de personalização.
- **E-mails diferentes para a mesma pessoa.** Se o login do seu painel usa um e-mail e seu usuário SSO outro, eles são dois assentos. Usar o mesmo e-mail em ambos os mescla.

### Verificando E Removendo Assentos

A página [Billing Analytics](https://fastcomments.com/auth/my-account/analytics/billing) lista suas contagens de `Moderators`, `SSO Admins` e `SSO Moderators`, e a prévia da próxima fatura mostra quanto elas custam. Remova as pessoas que você não precisa das páginas `Users` e `Moderators`, ou remova as flags de admin do seu payload SSO. Os assentos são contados quando a fatura é criada no final do mês, então um assento removido antes disso não é cobrado por aquele mês.

Veja [Adding Moderators](/guide-users.html#adding-moderators) e [Updating Administrator Permissions](/guide-users.html#updating-administrator-permissions) para saber como gerenciar ambos.