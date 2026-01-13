[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podemos ativar o suporte a spoilers definindo a flag **enableSpoilers** como true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a opção "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Quando o texto for destacado e o botão `SPOILER`, agora visível, for clicado, o texto será mascarado até que o usuário passe o mouse sobre ele. No modo escuro fazemos a mesma coisa, com cores diferentes que combinam melhor com o modo escuro.

Isso também é compatível com o editor WYSIWYG.