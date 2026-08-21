Existem duas maneiras de impedir que usuários comentem em seu site com o FastComments.

A primeira é se você já souber o e‑mail deles, pode inseri‑lo na página de <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">usuários banidos</a>.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista de usuários banidos em Moderação de Comentários, com os endereços de e‑mail banidos e um botão para adicionar um novo banimento'; title='Página de Usuários Banidos' app-screenshot-end]

Esta página pode ser acessada via Moderação de Comentários -> Usuários Banidos

Ao banir um usuário, podemos escolher um tipo, seja Permanente ou Banimento Sombrio Permanente:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Novo formulário de banimento com um campo de e‑mail e escolha do tipo de banimento: Permanente ou Banimento Sombrio Permanente'; title='Banindo um Usuário' app-screenshot-end]

A segunda maneira de banir um usuário é clicando no botão de banimento que está colocado em cada comentário na página de Moderação de Comentários.

Ao clicar no botão de banimento, serão apresentadas algumas opções, onde podemos especificar o tipo de banimento e a duração.

### Aliases de E‑mail

Ao banir um usuário por e‑mail, o FastComments ignora automaticamente aliases com `+`. Por exemplo, banir `user+alias@gmail.com` também banirá `user@gmail.com` e qualquer outra variação com `+` desse endereço, como `user+other@gmail.com`.

### Banimento Sombrio

Um banimento sombrio é um tipo de banimento que faz parecer que o comentário ou voto do usuário foi salvo com sucesso, quando na verdade não foi. Isso pode ser desejável em certas situações.

### Banimento via Endereço IP

A menos que um locatário opte por não participar, o FastComments oferece suporte ao banimento via IP armazenando uma versão hash do endereço IP do comentarista.

### Pesquisando Usuários Banidos

Quando sua lista crescer além de uma ou duas páginas, você pode refiná‑la usando a linha de pesquisa acima da tabela.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Linha de pesquisa na página de Usuários Banidos com um menu suspenso Pesquisar Por, um menu suspenso Correspondência e um campo de entrada Valor'; title='Pesquisando Usuários Banidos' app-screenshot-end]

Existem três controles:

- **Search By** escolhe em qual campo procurar: Qualquer Campo, E‑mail, Nome, Banido Por ou Banido Por Dizer. Os últimos quatro correspondem às colunas de mesmo nome na tabela.
- **Match** escolhe como comparar. **Contains** encontra seu valor em qualquer parte do campo, e **Equals** corresponde ao campo inteiro.
- **Value** é o texto a ser procurado.

Cada campo é comparado sem distinção entre maiúsculas e minúsculas, portanto pesquisar por `SPAMMER@EXAMPLE.COM` encontra um banimento armazenado como `spammer@example.com`.

Algumas coisas importantes a saber:

- **Banned For Saying** pesquisa o texto do comentário que resultou no banimento do usuário. É assim que você encontra todos os usuários banidos por uma frase específica.
- **Banned By** pesquisa o nome do moderador que aplicou o banimento, o que é útil para revisar as decisões de outro moderador.
- Banimentos curinga são armazenados com `*`, portanto uma pesquisa **Contains** por `bademail.com` encontra um banimento `*@bademail.com`.
- **Name** corresponde ao nome exibido na coluna Nome, portanto encontra um usuário mesmo que ele tenha alterado o nome desde o banimento, e mesmo que você tenha criado o banimento inserindo um endereço de e‑mail e nenhum nome tenha sido registrado na época. O nome registrado no banimento ainda corresponde, então pesquisar pelo nome antigo ou atual funciona.
- **Any Field** pesquisa o e‑mail, nome, moderador que aplicou o banimento e o texto do comentário banido juntos.

Sua pesquisa faz parte da URL da página, portanto você pode compartilhar uma lista filtrada com outros moderadores da mesma forma que compartilha outros links de moderação. Navegar pelos resultados mantém a pesquisa aplicada, iniciar uma nova pesquisa leva você de volta à primeira página, e **Clear** retorna à lista completa.