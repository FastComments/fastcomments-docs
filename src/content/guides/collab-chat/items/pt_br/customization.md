### Suporte ao Modo Escuro

### Modo Escuro Dinâmico

Se o modo escuro do seu site é controlado adicionando a classe `.dark` ao elemento body, a UI do Collab Chat respeitará isso automaticamente sem exigir reinicialização. Os estilos do widget são projetados para responder à presença dessa classe.

[inline-code-attrs-start title = 'Exemplo de CSS para Modo Escuro'; type = 'css'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
/* Seu CSS de modo escuro */
body.dark {
    background: #1a1a1a;
    color: #ffffff;
}
[inline-code-end]

### Estilização personalizada com CSS

Você pode personalizar a aparência dos destaques, janelas de chat e outros elementos usando CSS. O widget adiciona classes específicas que você pode direcionar em sua folha de estilo.

Os destaques de texto usam o sistema de estilização de balões de comentário do FastComments, então quaisquer personalizações aplicadas ao widget de comentários padrão também afetarão o Collab Chat.

### Personalização da Barra Superior

A barra superior mostra o número de usuários online e o número de discussões. Você pode personalizar sua posição fornecendo um elemento personalizado como `topBarTarget`:

[inline-code-attrs-start title = 'Localização personalizada da barra superior'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: document.getElementById('my-custom-header')
});
[inline-code-end]

Ou desativá-la completamente definindo-a como `null`:

[inline-code-attrs-start title = 'Desativar barra superior'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    topBarTarget: null
});
[inline-code-end]

### Comportamento em dispositivos móveis

Em telas com largura inferior a 768px, o Collab Chat alterna automaticamente para um layout otimizado para dispositivos móveis. As janelas de chat aparecem em tela cheia em vez de flutuarem ao lado do texto, e a demora na seleção é removida para uma interação mais imediata.

Esse comportamento é embutido e não requer nenhuma configuração. O widget detecta o tamanho da tela automaticamente e ajusta-se conforme necessário.

### Aparência das janelas de chat

As janelas de chat têm 410px de largura no desktop com uma seta de 16px apontando para o texto destacado. As janelas se posicionam automaticamente com base no espaço disponível na viewport, usando classes de posicionamento como `to-right`, `to-left`, `to-top` e `to-bottom`.

Você pode adicionar CSS personalizado para ajustar cores, fontes, espaçamento ou outras propriedades visuais dessas janelas. As janelas de chat usam a mesma estrutura de componentes que o widget padrão do FastComments, portanto herdam quaisquer personalizações globais que você aplicou.

### Localização

O Collab Chat oferece todas as mesmas opções de localização que o widget padrão do FastComments. Defina a opção `locale` para exibir o texto da UI em diferentes idiomas:

[inline-code-attrs-start title = 'Definir localidade'; type = 'javascript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
FastCommentsCollabChat(element, {
    tenantId: 'demo',
    locale: 'es'  // Espanhol
});
[inline-code-end]

O FastComments suporta dezenas de idiomas. A configuração de locale afeta todos os textos da UI, incluindo prompts, botões e texto de espaço reservado.

### Opções de personalização herdadas

Como o Collab Chat estende o widget de comentários padrão, ele herda todas as opções de personalização do widget base. Isso inclui classes CSS personalizadas, traduções personalizadas, personalização de avatar, formatação de datas e muito mais.

Consulte a documentação principal de personalização do FastComments para a lista completa de opções de personalização disponíveis.

### Trabalhando com fontes personalizadas

Se o seu site usa fontes personalizadas, a UI do Collab Chat herdará essas fontes do CSS da sua página. Pode ser necessário criar uma regra de personalização do widget e `@import` quaisquer fontes no CSS personalizado nessa regra se você quiser que a janela de chat flutuante use as mesmas fontes.