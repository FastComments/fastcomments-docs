Por padrão, o FastComments renderizará links assim: [https://exmaple.com](https://exmaple.com) - onde a URL do link se torna uma tag
âncora HTML clicável.

Alguns sites podem querer desativar isso, por exemplo para desencorajar golpistas. Fornecemos isso definindo a `Comment HTML Rendering Option` para `Links as Text`.

Isto pode ser personalizado sem código, na página de personalização do widget, para todo um domínio, ou página:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.show-advanced-option']; selector = '.comment-html-rendering-mode'; title='Render Links as Text' app-screenshot-end]