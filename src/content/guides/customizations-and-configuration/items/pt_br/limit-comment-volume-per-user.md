Por padrão, cada usuário pode enviar até `5 comments` no mesmo minuto.

Isso é rastreado por user id, anon user id, e ip address (hashed).

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; title='Limiting Comment Volume Per User' app-screenshot-end]

Observe que, se você estiver usando a comment creation API, pode querer enviar o `ip` original do usuário na requisição para o nosso backend para que a limitação de taxa seja aplicada
por usuário e não globalmente à sua conta.