[related-parameter-start name = 'enableWYSIWYG'; type = 'boolean'; related-parameter-end]

Por padrão, as funcionalidades de formatação no FastComments são feitas adicionando tags de âncora visíveis como `<b></b>` ao redor do seu texto. Clicar na barra de ferramentas
ou usar atalhos faz isso por você. No entanto, algumas comunidades podem querer optar por usar formatação sem tags de âncora. Isso é chamado de habilitar o
editor WYSIWYG (what you see is what you get). Este editor parece exatamente o mesmo que o padrão, exceto que ele carrega algum
código extra que permite aos usuários deixar o texto em negrito, sublinhado, etc., sem tags de âncora visíveis.

[code-example-start config = {enableWYSIWYG: true}; linesToHighlight = [6]; title = 'Enabling WYSIWYG Editing'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a opção "Enable Advanced Formatting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-wysiwyg']; selector = '.enable-search-label'; alt='Página de personalização do widget com a caixa de seleção Enable Advanced Formatting marcada para ativar o editor WYSIWYG'; title='Ativar WYSIWYG' app-screenshot-end]

---