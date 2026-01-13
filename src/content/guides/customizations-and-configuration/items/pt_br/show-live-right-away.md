[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Por padrão, os comentários ao vivo estão habilitados. Isso significa que se quaisquer comentários forem adicionados, excluídos, editados ou fixados, as alterações devem aparecer
para todos os usuários visualizando o fio de comentários ao mesmo tempo.

No entanto, por padrão esses novos comentários aparecerão sob um botão exibido dinamicamente com um texto similar a "Mostrar 2 novos comentários".

Se os novos comentários forem respostas diretamente à página, o botão será exibido na parte superior do fio de comentários. Se forem respostas a um comentário em particular, 
o botão será exibido sob esse comentário.

Isso é para evitar que o tamanho da página mude constantemente para o usuário, potencialmente causando frustração ao tentar pegar a barra de rolagem.

Para alguns casos de uso, como leilões ao vivo ou eventos online, esse não é o comportamento desejado - você pode querer que o widget de comentários seja
mais como uma "caixa de chat" onde os novos comentários "apareçam imediatamente".

Daí o nome da flag que habilita esse recurso: **showLiveRightAway**.

Podemos ativá-la da seguinte forma:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Isso pode ser personalizado sem código, na página de personalização do widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]

---