### Suporte a Modo Escuro

O Image Chat inclui suporte integrado ao modo escuro. Quando você define `hasDarkBackground: true` na sua configuração, as janelas de chat e os elementos da UI ajustam-se automaticamente para funcionar bem em fundos escuros.

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    hasDarkBackground: true
});
```

O estilo do modo escuro é aplicado às janelas de chat, aos marcadores quadrados e a todos os elementos interativos. Se o seu site possui um alternador de modo escuro, você pode reinicializar o widget quando o modo mudar, ou usar a abordagem de classe no body descrita abaixo.

### Modo Escuro Dinâmico

Se o modo escuro do seu site é controlado adicionando a classe `.dark` ao elemento body, a UI do Image Chat irá respeitar isso automaticamente sem requerer reinicialização. Os estilos do widget são projetados para responder à presença dessa classe.

```css
/* Seu CSS de modo escuro */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
```

### Estilização Personalizada com CSS

Você pode personalizar a aparência dos marcadores, das janelas de chat e de outros elementos usando CSS. O widget adiciona classes específicas que você pode direcionar em sua folha de estilos.

Os quadrados e janelas de chat usam o sistema de estilos de balões de comentário do FastComments, então quaisquer personalizações que você aplicou ao widget de comentários padrão também afetarão o Image Chat.

### Tamanho do Quadrado de Chat

A opção `chatSquarePercentage` controla o tamanho dos marcadores clicáveis. O padrão é 5% da largura da imagem:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    chatSquarePercentage: 7  // Quadrados maiores e mais visíveis
});
```

Valores menores criam marcadores mais sutis que se misturam à imagem. Valores maiores tornam os marcadores mais proeminentes e fáceis de clicar, especialmente em dispositivos móveis ou por motivos de acessibilidade.

### Comportamento em Dispositivos Móveis

Em telas com largura inferior a 768px, o Image Chat muda automaticamente para um layout otimizado para dispositivos móveis. As janelas de chat aparecem em fullscreen em vez de flutuarem ao lado dos marcadores, proporcionando melhor usabilidade em telas pequenas.

Os marcadores permanecem visíveis em suas posições responsivas na imagem. Os usuários podem tocar em qualquer marcador para abrir a interface de chat em tela cheia. Esse comportamento é integrado e não requer nenhuma configuração.

### Aparência da Janela de Chat

As janelas de chat têm 300px de largura no desktop com uma flecha de 16px apontando para o marcador. As janelas se posicionam automaticamente com base no espaço disponível da viewport, usando classes de posicionamento como `to-right`, `to-left`, `to-top`, e `to-bottom`.

Você pode adicionar CSS personalizado para ajustar cores, fontes, espaçamento ou outras propriedades visuais dessas janelas. As janelas de chat usam a mesma estrutura de componentes do widget padrão do FastComments, portanto herdam quaisquer personalizações globais que você aplicou.

### Inicialização Preguiçosa

As janelas de chat são inicializadas ao passar o mouse para usuários de desktop ou imediatamente quando criadas. Isso reduz a sobrecarga de carregamento inicial, renderizando a interface de chat somente quando os usuários realmente interagem com um marcador.

A inicialização preguiçosa ocorre de forma transparente. Os usuários não percebem qualquer atraso, mas o navegador não precisa renderizar dezenas de janelas de chat ocultas se você tiver muitos marcadores em uma imagem.

### Localização

O Image Chat suporta todas as mesmas opções de localização que o widget padrão do FastComments. Defina a opção `locale` para exibir o texto da UI em diferentes idiomas:

```javascript
FastCommentsImageChat(imageElement, {
    tenantId: 'demo',
    locale: 'fr'  // Francês
});
```

O FastComments suporta dezenas de idiomas. A configuração de locale afeta todo o texto da UI, incluindo prompts, botões e texto de placeholder.

### Opções de Personalização Herdadas

Como o Image Chat estende o widget de comentários padrão, ele herda todas as opções de personalização do widget base. Isso inclui classes CSS personalizadas, traduções personalizadas, customização de avatar, formatação de data e muito mais.

Veja a documentação principal de personalização do FastComments para a lista completa de opções de personalização disponíveis.

### Trabalhando com Fontes Personalizadas

Se o seu site usa fontes personalizadas, a UI do Image Chat herdará essas fontes do CSS da sua página. As janelas de chat são renderizadas dentro do DOM da sua página e respeitam as configurações de tipografia existentes.

Para melhores resultados, garanta que suas fontes personalizadas sejam carregadas antes de inicializar o Image Chat, ou aceite que pode haver um breve flash de texto sem estilo enquanto as fontes carregam.

### Design Visual dos Marcadores

Os marcadores quadrados têm um design visual sutil que os torna perceptíveis sem sobrecarregar a imagem. Você pode personalizar sua aparência com CSS se quiser um tratamento visual diferente.

Os marcadores incluem estados de hover que fornecem feedback quando os usuários movem o mouse sobre eles. Em dispositivos com touch, a interação por toque fornece feedback imediato abrindo a janela de chat.

---