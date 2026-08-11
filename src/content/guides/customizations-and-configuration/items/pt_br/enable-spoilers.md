---
[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podemos habilitar o suporte a spoilers definindo a flag **enableSpoilers** como true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Isso também pode ser feito sem código. Na página de personalização do widget, veja a opção "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Página de personalização do widget com a caixa de seleção Enable Spoilers marcada para adicionar o botão SPOILER ao editor'; title='Habilitar Spoilers' app-screenshot-end]

Quando o texto está destacado e o botão `SPOILER` agora visível é clicado, o texto será mascarado até que o usuário passe o mouse sobre ele. No modo escuro fazemos o mesmo, com cores diferentes que combinam melhor com o modo escuro.

Isso também é compatível com o editor WYSIWYG.

---