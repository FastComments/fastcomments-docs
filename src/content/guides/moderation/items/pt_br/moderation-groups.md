---
Os moderadores podem ser colocados em grupos para moderar diferentes páginas ou categorias de conteúdo.

Quando um moderador pertence a um ou mais grupos, ele verá apenas comentários desses grupos na página Moderar Comentários.

Por exemplo, digamos que administramos um site que exibe vídeos por categoria. Podemos querer ter moderadores diferentes para vídeos de Gato, Cachorro e Papagaio, então [vamos adicionar esses grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups).

[app-screenshot-start url='/auth/my-account/moderate-comments/moderation-groups?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderation-groups'; selector = '.content'; alt='Lista de grupos de moderação com os grupos Gato, Cachorro e Papagaio criados para cada categoria de vídeo'; title='Página de Grupos de Moderação' app-screenshot-end]

Ao adicionarmos um moderador, agora temos a opção de selecionar um ou mais grupos aos quais o moderador pertencerá:

[app-screenshot-start url='/auth/my-account/moderate-comments/moderator/new?demo=true'; linkUrl='/auth/my-account/moderate-comments/moderator/new'; selector = '.account-block'; alt='Formulário Adicionar Moderador com o seletor de grupo usado para atribuir o moderador a um ou mais grupos'; title='Adicionando um Moderador e Selecionando um Grupo' app-screenshot-end]

Finalmente, os comentários precisam ser vinculados a um ou mais grupos para que os moderadores corretos os vejam.

Isso pode ser configurado ao [adicionando alguns grupos](https://fastcomments.com/auth/my-account/moderate-comments/moderation-groups) e então especificar os IDs de `Moderation Group` correspondentes no widget de comentários, [conforme instruído aqui](/guide-customizations-and-configuration.html#moderation-group-ids).

---