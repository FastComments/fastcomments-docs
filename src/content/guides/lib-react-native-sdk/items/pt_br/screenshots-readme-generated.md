---
#### Tema: Erebus
![Tema: Erebus](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-erebus.PNG)
#### Tema: Padrão
![Tema: Padrão](images/sdk-images/lib-react-native-sdk--example-screenshots-skin-default.PNG)
#### Editor WYSIWYG nativo com suporte a imagens!
![Editor WYSIWYG nativo com suporte a imagens](images/sdk-images/lib-react-native-sdk--example-screenshots-native-wysiwyg.PNG)

### Editor de Texto Rico

This library uses [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) for rich text editing, which provides a powerful WYSIWYG editing experience. The same editor powers iOS, Android, and the web (via `react-native-web`), so the composer behaves consistently across every platform with a single implementation.

`react-native-enriched` requires the React Native New Architecture (Fabric) on native, and a bundler that resolves package `exports` conditions (Metro with package exports / RN 0.72+). Web support is currently experimental.

### Opções de Configuração

Esta biblioteca tem como objetivo suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

### Conceitos do FastComments

Os principais conceitos a conhecer para começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta no FastComments.com. `urlId` é onde os tópicos de comentários ficarão vinculados. Isso pode ser a URL de uma página, ou o id de um produto, o id de um artigo, etc.

### Notificações de Usuário

O FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis, podem ser desativadas globalmente ou no nível de uma notificação/comentário, e suportam inscrições por página para que os usuários possam assinar os tópicos de uma página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então periodicamente consultar por notificações não lidas e enviá-las ao usuário.

Veja [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber como obter e traduzir notificações não lidas do usuário.

### Navegador de GIFs

Por padrão, nenhuma seleção de imagem ou gif está habilitada. Veja [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para ver como suportar uploads de imagens e gifs. Há um Navegador de GIFs que anonimiza as buscas e as imagens fornecidas nesta biblioteca; você só precisa usá-lo.

### Desempenho

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo usado, se você identificar quaisquer problemas de desempenho. O desempenho é tratado como prioridade em todas as bibliotecas FastComments.
---