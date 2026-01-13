[related-parameter-start name = 'enableThirdPartyCookieBypass'; type = 'boolean'; related-parameter-end]

Para autenticação, o FastComments depende de cookies de terceiros habilitados no seu navegador. Sem eles, os usuários sempre terão que
deixar o e-mail para comentar (a menos que o campo de entrada de e-mail esteja oculto), e seus comentários sempre aparecerão como não verificados (por padrão).

Para contornar isso, você pode ativar o bypass de cookies de terceiros. 

Quando essa configuração estiver ativada, ela exibirá um pequeno popup com uma mensagem informando que o usuário está sendo autenticado. Esse popup
aparece sempre que o usuário interage com o widget de comentários; por exemplo, se ele deixar um comentário.

Podemos fazer isso no código definindo a flag **enableThirdPartyCookieBypass** como true:

[code-example-start config = {enableThirdPartyCookieBypass: true}; linesToHighlight = [6]; title = 'Enabling Third-Party Cookie Bypass'; code-example-end]

Também podemos configurar isso via a UI de Personalização do Widget, em `Enable Third-Party Cookie Popup`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.enable-third-party-cookie-bypass'; clickSelectors = ['.enable-third-party-cookie-bypass']; title='Enabling Third-Party Cookie Bypass' app-screenshot-end]

---