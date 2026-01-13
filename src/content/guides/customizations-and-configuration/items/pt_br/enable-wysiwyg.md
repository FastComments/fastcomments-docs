[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Por padrão, as funcionalidades de formatação no FastComments são feitas adicionando tags visíveis como `<b></b>` ao redor do seu texto. Clicar na barra de ferramentas
ou usar atalhos faz isso por você. No entanto, algumas comunidades podem querer optar por usar formatação sem tags visíveis. Isso é chamado de habilitar o
editor WYSIWYG (o que você vê é o que você obtém). Este editor parece exatamente igual ao padrão, exceto que carrega um código
extra que permite aos usuários colocar em negrito, sublinhar, etc., o texto deles sem tags visíveis.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a opção "Ativar Formatação Avançada".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; title='Enable WYSIWYG' app-screenshot-end]