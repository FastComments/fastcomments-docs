O FastComments funciona com sites apenas para membros usando o que é chamado de SSO, ou single-sign-on. Seus membros fazem login no seu site WordPress, mas
não precisam se preocupar em criar uma conta no FastComments, ou fazer login com redes sociais, para comentar. Se seus membros (incluindo administradores) estiverem conectados ao
seu site WordPress, eles poderão comentar. Seus administradores e moderadores poderão moderar comentários diretamente nas postagens do seu blog WordPress também.

<sup>(Optional)</sup> Lembre-se também de adicionar seus administradores a [Usuários & Administradores](https://fastcomments.com/auth/my-account/users) e moderadores a [Moderadores de Comentários](https://fastcomments.com/auth/my-account/moderate-comments/moderators)
para melhorar a experiência e habilitar o rastreamento de estatísticas para moderadores.

O SSO pode ser ativado acessando o painel do plugin e clicando em "Configurações SSO".

Você primeiro terá que ativar o recurso "Qualquer pessoa pode se registrar" do seu site.

Todas as informações do usuário são transferidas de forma transparente e segura para o FastComments cada vez que um usuário visualiza um tópico de comentários via [HMAC](https://en.wikipedia.org/wiki/HMAC).

Não é necessário executar nenhuma sincronização inicial ou contínua para copiar seus membros para os servidores do FastComments. Isso é feito automaticamente quando eles visualizam tópicos de comentários ao abrir uma postagem do blog.

## Nomes e Avatares

O plugin atualizará automaticamente o nome de exibição e o avatar do usuário em todos os seus comentários dentro do FastComments quando eles visualizarem
qualquer tópico de comentários. Avatares são suportados via Gravatar ou qualquer plugin de gerenciamento de avatar no WordPress, pois o plugin chamará `get_avatar_url`.