[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments permite que o usuário insira um comentário com quantas linhas quiser, até o limite de caracteres padrão.

No entanto, pode ser desejável limitar o usuário a inserir apenas uma única linha de texto. Alguns casos de uso exemplares incluem lances online ou chat ao vivo, para os quais o FastComments pode ser usado.

Nós habilitamos a flag **useSingleLineCommentInput** da seguinte forma:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Habilitar Entrada de Comentário de Linha Única'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a seção "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Caixa de seleção de entrada de comentário de linha única ativada na página de personalização do widget, limitando a entrada a uma linha'; title='Enable Single-Line Comment Input' app-screenshot-end]

Observe que os comentários em cada página para cada direção de ordenação são pré-computados, portanto todas as direções de ordenação têm o mesmo desempenho.