[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Para autenticação, o FastComments depende de cookies de terceiros estarem habilitados no seu navegador. Sem eles, os usuários sempre precisarão
deixar seu e‑mail para comentar (a menos que o campo de entrada de e‑mail esteja oculto), e seus comentários sempre aparecerão como não verificados (por padrão).

Para contornar isso, você pode habilitar a bypass de cookies de terceiros. 

Quando essa configuração está habilitada, ela causará um pequeno pop‑up que mostra uma mensagem indicando que o usuário está sendo autenticado. Esse pop‑up
é exibido sempre que o usuário interage com o widget de comentários; por exemplo, se ele deixar um comentário.

Podemos fazer isso no código definindo a flag **enableThirdPartyCookieBypass** como true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Habilitando Bypass de Cookie de Terceiros'; code-example-end]

Também podemos configurar isso via a UI de Personalização do Widget, em `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; alt='Página de personalização do widget com a caixa de seleção Enable Third-Party Cookie Popup marcada'; title='Habilitando Bypass de Cookie de Terceiros' app-screenshot-end]

---