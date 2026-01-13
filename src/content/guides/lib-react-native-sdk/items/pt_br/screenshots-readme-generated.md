---
#### Tema: Erebus
![Tema: Erebus](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-erebus.PNG)
#### Tema: Default
![Tema: Default](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/skin-default.PNG)
#### Editor WYSIWYG nativo com suporte a imagens!
![Editor WYSIWYG nativo com suporte a imagens](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/screenshots/native-wysiwyg.PNG)

### Editor de Texto Rico

Esta biblioteca usa o editor 10tap para funcionalidade de edição de texto rico, que fornece uma experiência WYSIWYG poderosa.

### Opções de Configuração

Esta biblioteca tem como objetivo suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

### Conceitos do FastComments

Os principais conceitos a serem compreendidos para começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta FastComments.com. `urlId` é onde os tópicos de comentários
serão vinculados. Isso pode ser a URL de uma página, ou um id de produto, um id de artigo, etc.

### Notificações de Usuário

O FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis,
podem ser desativadas globalmente ou a nível de notificação/comentário, e suportam assinaturas por página para que os usuários possam se inscrever em tópicos de uma
página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então periodicamente consultar por notificações não lidas e enviá-las para o usuário.

Veja [o exemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber como obter e traduzir notificações de usuário não lidas.

### Navegador de GIFs

Por padrão, nenhuma seleção de imagem ou gif está habilitada. Veja [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber como
suportar uploads de imagens e gifs. Há um Navegador de GIFs que anonimiza pesquisas e imagens fornecidas nesta biblioteca, você simplesmente precisa usá-lo.

### Desempenho

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo usado, se você identificar quaisquer problemas de desempenho. Desempenho é uma prioridade
em todas as bibliotecas FastComments.
---