[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

O número máximo de caracteres permitidos para serem inseridos no campo de entrada de comentário pode ser limitado pelo parâmetro **maxCommentCharacterLength**.

O padrão é 2000.

Itens como URLs de imagens não são incluídos na determinação do comprimento.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Limitar Tamanho do Comentário'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Campo de tamanho máximo de comentário na página de personalização do widget, usado para limitar quantos caracteres um comentário pode conter'; title='Limitar Tamanho do Comentário' app-screenshot-end]