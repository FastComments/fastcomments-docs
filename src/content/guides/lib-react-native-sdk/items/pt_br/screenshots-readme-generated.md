Comentários encadeados ao vivo com avatares, respostas aninhadas, votos e o composer rich-text embutido, além de um tema escuro e um preset de chat ao vivo (mostrado aqui renderizado via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentários ao Vivo</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Comentário ao vivo, tema claro"/></td>
    <td align="center"><b>Tema Escuro</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Comentário ao vivo, tema escuro"/></td>
    <td align="center"><b>Chat ao Vivo</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Preset de chat ao vivo"/></td>
  </tr>
</table>

### Rich Text Editor

Esta biblioteca usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para edição rich text, que fornece uma experiência WYSIWYG poderosa. O mesmo editor alimenta iOS, Android e a web (via `react-native-web`), então o composer se comporta de forma consistente em todas as plataformas com uma única implementação.

`react-native-enriched` requer a Nova Arquitetura do React Native (Fabric) no nativo (o padrão desde o RN 0.76, opt-in no RN 0.72-0.75), e um bundler que resolva as condições de `exports` do pacote. Este SDK é desenvolvido e testado contra RN 0.81 / React 19. O mesmo editor também roda na web através do `react-native-web`; a build web do editor enriched ainda é marcada como experimental upstream.

### Widgets

O SDK inclui três widgets, espelhando o FastComments Android SDK:

- `FastCommentsLiveCommenting` - comentários encadeados com votos, respostas, paginação, menções, notificações e atualizações ao vivo.
- `FastCommentsLiveChat` - um preset de chat sobre o mesmo motor: mensagens cronológicas com as novas no final, o composer abaixo da lista, uma faixa de cabeçalho ao vivo (ponto de conexão + contagem de usuários), histórico infinito carregado ao rolar para cima, auto-scroll para novas mensagens, sem votos ou encadeamento de respostas. Cada preset pode ser sobrescrito via `config`.
- `FastCommentsFeed` - um feed social com composer de post, mídia, reações, seguir e banners ao vivo de novos posts.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Theming

A aparência padrão é gerada a partir de um conjunto de tokens semânticos de design (`FastCommentsTheme`): cores, espaçamentos, raio, tamanhos de fonte, pesos de fonte e tamanhos de avatar. Passe overrides parciais de tokens (tipados como `FastCommentsThemeOverrides`) através da prop `theme` em qualquer widget e toda a árvore de estilos será reestilizada de forma consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

O modo escuro está a apenas um conjunto de tokens de distância:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

A prop `styles` ainda aceita uma árvore bruta `IFastCommentsStyles` para controle cirúrgico. Quando `theme` e `styles` são fornecidos, os estilos explícitos ganham sobre a árvore themed; quando somente `styles` é fornecido, ele substitui os padrões inteiramente (o comportamento original, então integrações e skins existentes não são afetadas). `setupDarkModeSkin` está obsoleto em favor da prop `theme`.

### Configuration Options

Esta biblioteca busca suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

Além daquelas, o React Native adiciona algumas opções específicas do SDK via `FastCommentsRNConfig`:

- `hideTopBar` - oculta a faixa do usuário logado / sino de notificações mostrada acima do composer.
- `usePressToEdit` - pressione e segure um comentário para abrir seu menu.
- `disableDownVoting` - oculta botões de down-vote.
- `renderCommentInline` - renderiza as informações do comentarista dentro do mesmo bloco HTML que o conteúdo do comentário.
- `renderLikesToRight` - move a área de voto/curtida para a direita do comentário em vez de abaixo dele.
- `renderDateBelowComment` - renderiza a data abaixo do comentário.
- `showLiveStatus` - mostra a faixa de cabeçalho no estilo chat "Live" + contagem de usuários acima dos comentários.
- `useInlineSubmitButton` - renderiza o botão de envio como um ícone dentro do composer.
- `countAboveToggle` - com `useShowCommentsToggle`, quantos comentários renderizar acima do toggle "Mostrar comentários".
- `preserveFeedScrollPosition` - `FastCommentsFeed` lembra seu offset de rolagem entre unmount/remount (padrão true).

### FastComments Concepts

Os principais conceitos para ficar atento ao começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta FastComments.com. `urlId` é onde os tópicos de comentários serão vinculados. Isso pode ser uma URL de página, ou um id de produto, um id de artigo, etc.

### User Notifications

FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis, podem ser desativadas globalmente ou ao nível de notificação/comentário, e suportam assinaturas a nível de página para que os usuários possam assinar threads de uma página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então periodicamente checar por notificações não lidas e enviá-las ao usuário.

Veja [the example AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para como obter e traduzir notificações não lidas do usuário.

### Gif Browser

Por padrão, nenhuma seleção de imagem ou gif está habilitada. Veja [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para como suportar uploads de imagem e gif. Existe um Gif Browser que anonimiza pesquisas e imagens fornecidas nesta biblioteca, você simplesmente precisa usá-lo.

### Performance

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo usado, se você identificar quaisquer problemas de performance. Performance é uma prioridade de primeira classe em todas as bibliotecas FastComments.