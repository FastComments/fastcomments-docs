Live threaded commenting with avatars, nested replies, votes, and the built-in rich-text composer, plus a dark theme and a live-chat preset (shown here rendered via `react-native-web`):

<table>
  <tr>
    <td align="center"><b>Comente ao Vivo</b><br/><img src="./demo-screenshots/light.png" width="260" alt="Comente ao vivo, tema claro"/></td>
    <td align="center"><b>Tema Escuro</b><br/><img src="./demo-screenshots/dark.png" width="260" alt="Comente ao vivo, tema escuro"/></td>
    <td align="center"><b>Chat ao Vivo</b><br/><img src="./demo-screenshots/chat.png" width="260" alt="Pré-configuração de chat ao vivo"/></td>
  </tr>
</table>

### Editor de Texto Rico

Esta biblioteca usa [`react-native-enriched`](https://github.com/software-mansion/react-native-enriched) para edição de texto rico, que fornece uma experiência de edição WYSIWYG poderosa. O mesmo editor alimenta iOS, Android e a web (via `react-native-web`), então o compositor se comporta de forma consistente em todas as plataformas com uma única implementação.

`react-native-enriched` requer a New Architecture do React Native (Fabric) nativa (padrão desde RN 0.76, opt‑in no RN 0.72‑0.75), e um bundler que resolve as condições de `exports` do pacote. Este SDK é desenvolvido e testado contra RN 0.81 / React 19. O mesmo editor também funciona na web através do `react-native-web`; a build web do enriched editor ainda está marcada como experimental upstream.

### Widgets

O SDK entrega três widgets, espelhando o FastComments Android SDK:

- `FastCommentsLiveCommenting` – comentários em árvore com votos, respostas, paginação, menções, notificações e atualizações ao vivo.
- `FastCommentsLiveChat` – um preset de chat sobre o mesmo motor: mensagens cronológicas com as novas na parte inferior, compositor abaixo da lista, faixa de cabeçalho ao vivo (ponto de conexão + contagem de usuários), histórico infinito carregado ao rolar para cima, rolagem automática para novas mensagens, sem votos ou encadeamento de respostas. Cada preset pode ser sobrescrito via `config`.
- `FastCommentsFeed` – um feed social com compositor de post, mídia, reações, follows e banners ao vivo de novos posts.

```tsx
    <FastCommentsLiveChat config=\{{ tenantId: 'demo', urlId: 'my-room' }}/>
```

### Tematização

A aparência padrão é gerada a partir de um conjunto de tokens de design semânticos (`FastCommentsTheme`): cores, espaçamentos, raio, tamanhos de fonte, pesos de fonte e tamanhos de avatar. Passe sobrescritas parciais de token (tipadas `FastCommentsThemeOverrides`) através da prop `theme` em qualquer widget e toda a árvore de estilos será re‑estilizada de forma consistente:

```tsx
    <FastCommentsLiveCommenting config={config} theme=\{{ colors: { primary: '#FF5500' } }}/>
```

O modo escuro está a um conjunto de tokens de distância:

```tsx
    import { getDarkTheme } from 'fastcomments-react-native-sdk';

    <FastCommentsLiveCommenting config={config} theme={getDarkTheme()}/>
```

A prop `styles` ainda aceita uma árvore crua `IFastCommentsStyles` para controle cirúrgico. Quando `theme` e `styles` são fornecidos, os estilos explícitos prevalecem sobre a árvore tematizada; quando apenas `styles` é fornecido, ele substitui totalmente os padrões (o comportamento original, então integrações e skins existentes permanecem inalterados). `setupDarkModeSkin` está obsoleta em favor da prop `theme`.

### Opções de Configuração

Esta biblioteca pretende suportar todas as opções de configuração definidas em [fastcomments-typescript](https://github.com/FastComments/fastcomments-typescript/blob/main/src/fast-comments-comment-widget-config.ts), assim como a implementação web.

Além dessas, o React Native adiciona algumas opções específicas do SDK via `FastCommentsRNConfig`:

- `hideTopBar` – oculta a faixa de usuário logado / sino de notificações mostrada acima do compositor.
- `usePressToEdit` – pressione e segure um comentário para abrir seu menu.
- `disableDownVoting` – oculta os botões de voto negativo.
- `renderCommentInline` – renderiza informações do comentarista dentro do mesmo bloco HTML do conteúdo do comentário.
- `renderLikesToRight` – move a área de voto/curtir para a direita do comentário em vez de abaixo dele.
- `renderDateBelowComment` – renderiza a data abaixo do comentário.
- `showLiveStatus` – mostra a faixa de cabeçalho estilo chat “Live” + contagem de usuários acima dos comentários.
- `useInlineSubmitButton` – renderiza o botão de envio como ícone dentro do compositor.
- `countAboveToggle` – com `useShowCommentsToggle`, quantos comentários são renderizados acima da alternância “Show Comments”.
- `preserveFeedScrollPosition` – `FastCommentsFeed` lembra seu deslocamento de rolagem entre desmontagens/montagens (padrão true).

### Conceitos do FastComments

Os principais conceitos a conhecer para começar são `tenantId` e `urlId`. `tenantId` é o identificador da sua conta FastComments.com. `urlId` é onde os tópicos de comentário serão vinculados. Isso pode ser um URL de página, um ID de produto, um ID de artigo, etc.

### Localização

Todo o texto voltado ao usuário nesses widgets (rótulos de botões, placeholders, estados vazios, datas relativas como “5 minutos atrás”, mensagens de erro, etc.) é **controlado pelo servidor**. Os componentes não codificam strings em inglês; eles renderizam as traduções que o FastComments fornece para o locale solicitado.

Para solicitar um locale, defina `locale` na sua configuração:

```ts
const config = {
    tenantId: 'your-tenant-id',
    urlId: 'some-page',
    locale: 'de_de', // de_de, fr_fr, ja_jp, es_es, etc.
};
```

Quando nenhum `locale` é definido, o FastComments serve o idioma padrão do tenant.

**Editando o texto:** as traduções são gerenciadas no seu painel do FastComments, não neste SDK. Para mudar a redação, sobrescreva o copy padrão ou adicione um idioma, editando as traduções da sua conta no painel – a alteração é aplicada nos widgets automaticamente sem necessidade de liberar uma nova versão do app. O SDK não entrega fallback em inglês, então qualquer chave que você deixe vazia no painel renderiza vazia; mantenha as chaves preenchidas para todo locale que você oferece suporte.

### Notificações de Usuário

O FastComments suporta notificações para [muitos cenários](https://docs.fastcomments.com/guide-notifications.html). As notificações são configuráveis, podem ser desativadas globalmente ou por nível de notificação/comentário, e suportam assinaturas ao nível da página para que usuários possam assinar tópicos de uma página ou artigo específico.

Por exemplo, é possível usar Secure SSO para autenticar o usuário e então sondar periodicamente por notificações não lidas e enviá‑las ao usuário.

Veja [o exemplo AppNotificationSecureSSO](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppNotificationsSecureSSO.tsx) para saber como obter e traduzir notificações de usuário não lidas.

### Navegador de GIFs

Por padrão, nenhuma seleção de imagem ou gif está habilitada. Consulte [example/src/AppCommentingImageSelection.tsx](https://github.com/FastComments/fastcomments-react-native-sdk/blob/main/example/src/AppCommentingImageSelection.tsx) para saber como suportar uploads de imagem e gif. Existe um Navegador de GIFs que anonimiza buscas e imagens fornecidas nesta biblioteca, basta utilizá‑lo.

### Performance

Por favor, abra um ticket com um exemplo para reproduzir, incluindo o dispositivo usado, se você identificar algum problema de performance. Performance é um cidadão de primeira classe de todas as bibliotecas FastComments.