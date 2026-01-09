Existem duas maneiras de banir usuários de comentar no seu site com o FastComments.

A primeira é: se você já conhece o e-mail deles, você pode inseri-lo na página <a href="/auth/my-account/moderate-comments/banned-users" target="_blank">usuários banidos</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Esta página pode ser acessada via Moderar Comentários -> Usuários Banidos

Quando formos banir um usuário, podemos escolher um tipo: Permanente ou Shadow Ban Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

A segunda maneira de banir um usuário é clicando no botão de banimento que fica em cada comentário na página Moderação de Comentários.

Ao clicarmos no botão de banimento, serão apresentadas algumas opções, onde podemos especificar o tipo e a duração do banimento.

### Shadow Bans

Um shadow-ban é um tipo de banimento que faz parecer que o comentário ou voto do usuário foi salvo com sucesso, quando na verdade não foi. Isso pode ser desejável em certas situações.

### Banning Via IP Address

A menos que um tenant deseje optar por não participar, o FastComments suporta banimento por IP armazenando uma versão hash do endereço IP do autor do comentário.