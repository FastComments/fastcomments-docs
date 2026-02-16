#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Padrão
![Tema: Padrão](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Editor WYSIWYG nativo com suporte a imagens!
![Editor WYSIWYG nativo com suporte a imagens](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor de Texto Rico

Esta biblioteca utiliza o editor 10tap para a funcionalidade de edição de texto rico, que fornece uma experiência poderosa de edição WYSIWYG.

### Opções de Configuração

Esta biblioteca tem como objetivo suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

### Conceitos do FastComments

Os principais conceitos a serem considerados para começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta FastComments.com. `urlId` é onde os threads de comentários estarão vinculados. Isso pode ser a URL de uma página, ou um id de produto, um id de artigo, etc.

### Notificações de Usuário

O FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis,
podem ser desativadas globalmente ou no nível de notificação/comentário, e suportam assinaturas ao nível da página para que os usuários possam se inscrever em threads de uma
página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então consultar periodicamente as notificações não lidas e enviá-las ao usuário.

Veja [o exemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) sobre como obter e traduzir notificações não lidas do usuário.

### Navegador de GIFs

Por padrão, nenhuma seleção de imagem ou GIF está habilitada. Veja [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber como suportar uploads de imagens e GIFs. Há um Navegador de GIFs que anonimiza pesquisas e imagens fornecidas nesta biblioteca, você simplesmente precisa usá-lo.

### Desempenho

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo utilizado, se identificar quaisquer problemas de desempenho. O desempenho é tratado como prioridade em todas as bibliotecas FastComments.