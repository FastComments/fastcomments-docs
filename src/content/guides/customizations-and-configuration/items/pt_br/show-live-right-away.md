[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Por padrão, a postagem ao vivo está habilitada. Isso significa que se quaisquer comentários forem adicionados, excluídos, editados ou fixados, as alterações devem aparecer
para todos os usuários que visualizam o thread de comentários ao mesmo tempo.

No entanto, por padrão esses novos comentários aparecerão sob um botão exibido dinamicamente com texto semelhante a "Mostrar 2 Novos Comentários".

Se os novos comentários forem respostas diretamente à página, o botão será exibido no topo do thread de comentários. Se forem respostas a um comentário específico, 
o botão será exibido abaixo desse comentário.

Isso serve para evitar que o tamanho da página mude constantemente para o usuário, potencialmente causando frustração ao tentar pegar a barra de rolagem.

Para alguns casos de uso, como leilões ao vivo ou eventos online, esse não é o comportamento desejado – você pode querer que o widget de comentários seja
mais como uma caixa de "chat" onde novos comentários "aparecem imediatamente".

Portanto, o nome da flag que habilita esse recurso: **showLiveRightAway**.

Podemos ativá-la da seguinte forma:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Mostrar Comentários ao Vivo Imediatamente'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; alt='Configuração de colapso de comentários ao vivo ativada para que novos comentários apareçam instantaneamente em vez de ficarem atrás de um botão'; title='Mostrar Comentários ao Vivo Imediatamente' app-screenshot-end]