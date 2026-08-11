---
Por padrão, cada usuário pode enviar até `5 comentários` no mesmo minuto.

Isso é rastreado por ID de usuário, ID de usuário anônimo e endereço IP (hash).

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comments-per-minute'; alt='Campo de máximo de comentários por minuto na página de personalização do widget, definido como 5 por padrão'; title='Limitando o Volume de Comentários por Usuário' app-screenshot-end]

Observe que, se você estiver usando a API de criação de comentários, pode querer passar o endereço `ip` original do usuário na solicitação ao nosso backend para que a limitação de taxa seja aplicada
por usuário e não globalmente à sua conta.

---