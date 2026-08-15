Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comentário ao Vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-light.png" width="260" alt="Comentário ao vivo, tema claro"/></td>
    <td align="center"><b>Tema Escuro</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-dark.png" width="260" alt="Comentário ao vivo, tema escuro"/></td>
    <td align="center"><b>Chat ao Vivo</b><br/><img src="images/sdk-images/lib-react-native-sdk--demo-screenshots-chat.png" width="260" alt="Preset de chat ao vivo"/></td>
  </tr>
</table>

### Editor de Texto Enriquecido

Esta biblioteca usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para edição de texto rico, que fornece uma experiência de edição WYSIWYG poderosa. O mesmo editor alimenta iOS, Android e a web (via `react-native-web`), portanto o compositor se comporta de forma consistente em todas as plataformas com uma única implementação.

`react-native-enriched` requer a New Architecture do React Native (Fabric) nativo (padrão desde RN 0.76, opcional nas versões RN 0.72-0.75) e um bundler que resolve as condições de `exports` do pacote. Este SDK foi desenvolvido e testado contra RN 0.81 / React 19. O mesmo editor também funciona na web através do `react-native-web`; a compilação web do editor enriched ainda está marcada como experimental upstream.

### Widgets

O SDK inclui três widgets, espelhando o SDK Android da FastComments:

- `FastCommentsLiveCommenting` - comentário em tópicos com votos, respostas, paginação, menções, notificações e atualizações em tempo real.
- `FastCommentsLiveChat` - um preset de chat sobre o mesmo motor: mensagens cronológicas com as novas na parte inferior, o compositor abaixo da lista, uma faixa de cabeçalho ao vivo (ponto de conexão + contagem de usuários), histórico infinito carregado ao rolar para cima, rolagem automática para novas mensagens, sem votos ou encadeamento de respostas. Cada preset pode ser sobrescrito via `config`.
- `FastCommentsFeed` - um feed social com compositor de post, mídia, reações, seguidores e banners ao vivo de novos posts.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematização

A aparência padrão é gerada a partir de um conjunto de tokens de design semânticos (`FastCommentsTheme`): cores, espaçamento, raio, tamanhos de fonte, pesos de fonte e tamanhos de avatar. Passe substituições parciais de tokens (tipadas `FastCommentsThemeOverrides`) através da propriedade `theme` em qualquer widget e toda a árvore de estilos será reestilizada de forma consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

O modo escuro está a um conjunto de tokens de distância:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

A propriedade `styles` ainda aceita uma árvore bruta `IFastCommentsStyles` para controle cirúrgico. Quando `theme` e `styles` são fornecidos, os estilos explícitos prevalecem sobre a árvore tematizada; quando apenas `styles` é fornecido, ele substitui totalmente os padrões (o comportamento original, portanto integrações e skins existentes não são afetados). `setupDarkModeSkin` está obsoleto em favor da propriedade `theme`.

### Opções de Configuração

Esta biblioteca tem como objetivo suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

Além dessas, o React Native adiciona algumas opções específicas do SDK via `FastCommentsRNConfig`:

- `hideTopBar` - ocultar a faixa de usuário logado / sino de notificação exibida acima do compositor.
- `usePressToEdit` - pressionar e segurar um comentário para abrir seu menu.
- `disableDownVoting` - ocultar botões de voto negativo.
- `renderCommentInline` - renderizar informações do comentarista dentro do mesmo bloco HTML do conteúdo do comentário.
- `renderLikesToRight` - mover a área de voto/curtida para a direita do comentário ao invés de abaixo dele.
- `renderDateBelowComment` - renderizar a data abaixo do comentário.
- `showLiveStatus` - exibir a faixa de cabeçalho estilo chat "Live" + contagem de usuários acima dos comentários.
- `useInlineSubmitButton` - renderizar o botão de envio como um ícone dentro do compositor.
- `countAboveToggle` - com `useShowCommentsToggle`, quantos comentários são renderizados acima da alternância "Mostrar Comentários".
- `preserveFeedScrollPosition` - `FastCommentsFeed` lembra seu deslocamento de rolagem entre desmontagens/montagens (padrão true).

### Conceitos do FastComments

Os principais conceitos a conhecer para começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta FastComments.com. `urlId` é onde os tópicos de comentários serão vinculados. Isso pode ser uma URL de página, um ID de produto, um ID de artigo, etc.

### Localização

Todo o texto voltado ao usuário nesses widgets (rótulos de botões, placeholders, estados vazios, datas relativas como "há 5 minutos", mensagens de erro, etc.) é **controlado pelo servidor**. Os componentes não codificam strings em inglês; eles renderizam as traduções que a FastComments fornece para o locale solicitado.

Para solicitar um locale, defina `locale` em sua configuração:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Quando nenhum `locale` é definido, a FastComments fornece o idioma padrão do tenant.

**Editando o texto:** as traduções são gerenciadas no painel da sua FastComments, não neste SDK. Para mudar a redação, sobrescreva o texto padrão ou adicione um idioma, edite as traduções da sua conta no painel – a alteração é captada pelos widgets automaticamente sem necessidade de liberar uma nova versão do app. O SDK não inclui fallback em inglês, portanto qualquer chave que você deixar vazia no painel será renderizada como vazia; mantenha as chaves preenchidas para cada locale que você suportar.

### Notificações de Usuário

FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis, podem ser desativadas globalmente ou em nível de notificação/comentário, e suportam assinaturas em nível de página para que os usuários possam assinar tópicos de uma página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então periodicamente consultar notificações não lidas e enviá‑las ao usuário.

Veja [o exemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber como obter e traduzir notificações de usuário não lidas.

### Navegador de GIF

Por padrão, nenhuma seleção de imagem ou gif está habilitada. Veja [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber como suportar uploads de imagem e gif. Existe um Navegador de GIF que anonimiza buscas e imagens fornecidas nesta biblioteca, você simplesmente precisa usá‑lo.

### Performance

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo usado, se você identificar algum problema de desempenho. O desempenho é um aspecto de primeira classe em todas as bibliotecas FastComments.