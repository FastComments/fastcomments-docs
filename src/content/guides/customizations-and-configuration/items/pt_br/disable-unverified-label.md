[related-parameter-start name = 'disableUnverifiedLabel'; type = 'boolean'; related-parameter-end]

Por padrão, o FastComments exibirá um rótulo "Comentário Não Verificado" para comentários que foram deixados para um usuário que
tem uma sessão de navegador não verificada. Leia mais sobre comentários não verificados [aqui](https://docs.fastcomments.com/guide-comment-vote-verification.html).

[code-example-start config = {disableUnverifiedLabel: true}; linesToHighlight = [6]; title = 'Disable The Unverified Label'; code-example-end]

Além disso, esse recurso pode ser usado, sem escrever código, na interface de Personalização:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-unverified-comment-label']; selector = '.disable-unverified-comment-label'; title='Disable The Unverified Label' app-screenshot-end]

---