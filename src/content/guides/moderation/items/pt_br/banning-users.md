Existem duas maneiras de banir usuários de comentar em seu site com o FastComments.

A primeira é se você já souber o e‑mail deles, pode inseri‑lo na página de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuários banidos</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista de usuários banidos em Moderar Comentários, com os endereços de e‑mail banidos e um botão para adicionar uma nova proibição'; title='Página de Usuários Banidos' app-screenshot-end]

Esta página pode ser acessada via Moderar Comentários -> Usuários Banidos

Ao banir um usuário, podemos escolher um tipo, seja Permanente ou Proibição Sombra Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Novo formulário de proibição com um campo de e‑mail e uma escolha de tipo de proibição entre Permanente ou Proibição Sombra Permanente'; title='Banindo um Usuário' app-screenshot-end]

A segunda maneira de banir um usuário é clicando no botão de banimento que está colocado em cada comentário na página de Moderação de Comentários.

Ao clicar no botão de banimento, serão apresentadas algumas opções, onde podemos especificar o tipo e a duração da proibição.

### Aliases de E‑mail

Ao banir um usuário por e‑mail, o FastComments ignora automaticamente aliases com `+`. Por exemplo, banir `user+alias@gmail.com` também banirá `user@gmail.com` e qualquer outra variação com `+` desse endereço, como `user+other@gmail.com`.

### Proibições Sombra

Uma proibição sombra é um tipo de banimento que faz parecer que o comentário ou voto do usuário foi salvo com sucesso, quando na verdade não foi. Isso pode ser desejável em certas situações.

### Banimento via Endereço IP

A menos que um locatário deseje optar por não participar, o FastComments oferece suporte ao banimento via IP armazenando uma versão hash do endereço IP do comentarista.

---