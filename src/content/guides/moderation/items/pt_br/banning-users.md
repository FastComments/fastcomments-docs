Existem duas maneiras de banir usuários de comentar em seu site com o FastComments.

A primeira é se você já conhece o e-mail deles, você pode inseri-lo na <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuários banidos</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; title='The Banned Users Page' app-screenshot-end]

Esta página pode ser acessada via Moderate Comments -> Banned Users

Quando vamos banir um usuário, podemos escolher um tipo, seja Permanent ou Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; title='Banning a User' app-screenshot-end]

A segunda forma de banir um usuário é clicando no botão de banir que está colocado em cada comentário na página de Comment Moderation.

Quando clicamos no botão de banir, serão apresentadas algumas opções, onde podemos especificar o tipo de banimento e a duração.

### Email Aliases

Ao banir um usuário por e-mail, o FastComments ignora automaticamente `+` aliases. Por exemplo, banir `user+alias@gmail.com` também banirá `user@gmail.com` e qualquer outra variação com `+` desse endereço, como `user+other@gmail.com`.

### Shadow Bans

Um shadow-ban é um tipo de banimento que faz parecer que o comentário ou voto do usuário foi salvo com sucesso, quando na verdade não foi. Isso pode ser desejável em certas situações.

### Banning Via IP Address

A menos que um tenant deseje optar por não participar, o FastComments suporta banimento via IP armazenando uma versão hashed do endereço IP do comentarista.