[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

O número máximo de caracteres permitidos no campo de entrada de comentário pode ser limitado pelo parâmetro **maxCommentCharacterLength**.

O padrão é 2000.

Itens como URLs de imagens não são incluídos no cálculo do comprimento.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limit Comment Length'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; title='Limit Comment Length' app-screenshot-end]

---