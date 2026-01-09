FastComments User Badges são configurados por administradores com a permissão `Customize Data`.

Isto é feito via [Personalizar -> Insígnias](https://fastcomments.com/auth/my-account/configure-badges) no seu painel de administração.

Quando um usuário recebe uma insígnia, ela é exibida em seu perfil e em seus comentários.

Ao adicionar uma insígnia podemos configurar um `Display Label`, que é o nome que o usuário vê associado à insígnia. Por exemplo, se adicionarmos uma insígnia `Comment Count`
provavelmente não queremos mostrar esse nome técnico porque é muito sem graça. Podemos chamá-la de `Super Member` ou algo semelhante. Insígnias também podem se empilhar e substituir umas às outras, como cobriremos
mais adiante neste documento.

Insígnias também têm limites configuráveis.

As insígnias podem ser criadas e, depois, desativadas desmarcando `Enabled`. Desativar uma insígnia significa que ela não será mais concedida automaticamente, e não aparecerá no menu Award Manual Badge, mas
os usuários manterão a insígnia.

### Tipos de Exibição de Insígnias

As insígnias podem ser imagens ou insígnias de texto, que suportam algumas estilizações básicas (cor do texto, cor de fundo e cor da borda). Você também pode estilizar insígnias via CSS.

Insígnias em imagem podem ser GIFs para mostrar insígnias animadas.

### Dica - Não Remova Insígnias!

Os usuários adoram insígnias. Eles tendem a se importar muito com elas, mesmo que seja um bug que você adicionou por engano, ou que você queira trocar o ícone da insígnia.

Se aprendemos alguma coisa, é que é extremamente difícil tirar algo dos usuários. Remover uma insígnia porque você, como proprietário do site, não
gosta mais dela, ou quer fazer mudanças, pode resultar em uma multidão muito irritada de usuários que de repente deixam seu site por frustração. Por essa razão
`Delete` nem era uma opção nos primeiros meses em que lançamos esse recurso - no entanto acabamos tendo que adicioná-la. Mas por favor, use o excluir com cautela. Temos
visto muitos usuários de longa data, com vários anos, ficarem muito frustrados e abandonarem suas comunidades porque administradores decidiram excluir uma insígnia.

Se você precisa parar de usar uma insígnia, sugerimos que simplesmente a desative para que os usuários mantenham sua insígnia.

### Reprocessamento de Insígnias

Quando uma insígnia é adicionada ou alterada, o sistema verificará retroativamente qualquer pessoa que tenha interagido com seu site para ver se ela deve receber a insígnia. Isso ficará
visível na página Insígnias no painel de administração, pois um spinner será exibido no lugar do número de usuários que possuem a insígnia. Isso acontece porque o número de usuários
está sendo determinado.

### Ver quem possui uma Insígnia

Na lista de Insígnias cada link tem a opção `View Users` para mostrar a lista de usuários que ganharam ou receberam manualmente uma insígnia.