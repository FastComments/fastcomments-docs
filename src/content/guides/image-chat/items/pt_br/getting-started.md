### Casos de Uso

O Image Chat funciona muito bem para feedback de design, onde equipes precisam discutir elementos específicos em mockups ou capturas de tela. Sites de avaliação de produtos podem permitir que clientes discutam recursos específicos visíveis em fotos de produtos. Plataformas educacionais podem usá-lo para discutir diagramas, mapas ou imagens científicas. Galerias de fotos podem se tornar interativas com comentários específicos por localização. Sites imobiliários podem permitir que visitantes discutam recursos específicos visíveis em fotos de imóveis.

### Início Rápido

Começar com o Image Chat é simples. Você precisa do script do FastComments Image Chat, um elemento de imagem ou contêiner com uma imagem, e um objeto de configuração com seu Tenant ID.

### Instalação

Adicione o script do Image Chat à sua página:

[inline-code-attrs-start title = 'Carregando o Script do Image Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>
[inline-code-end]

### Implementação Básica

Aqui está um exemplo mínimo:

[inline-code-attrs-start title = 'Implementação Básica do Image Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>Minha Galeria de Imagens com Image Chat</title>
</head>
<body>
    <!-- Sua imagem -->
    <img id="my-image" src="https://fastcomments.com/images/image-chat-demo-1.jpg" alt="Pretty Trail" />

    <!-- Carregar o script do Image Chat -->
    <script src="https://cdn.fastcomments.com/js/embed-image-chat.min.js"></script>

    <!-- Inicializar o Image Chat -->
    <script>
        FastCommentsImageChat(document.getElementById('my-image'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Substitua `'demo'` pelo seu Tenant ID real do FastComments, caso ainda não o tenha feito, que você pode encontrar no seu [painel do FastComments](https://fastcomments.com/auth/my-account).

### Como Funciona

Uma vez inicializado, os usuários podem clicar em qualquer lugar da imagem. Quando um clique ocorre, um marcador visual quadrado aparece naquele local e uma janela de chat se abre. Outros usuários podem ver todos os marcadores e clicar neles para visualizar ou participar dessas discussões. Todas as discussões são sincronizadas em tempo real entre todos os visitantes.

O widget usa posicionamento baseado em porcentagem, então os marcadores permanecem no local correto mesmo quando a imagem é redimensionada ou visualizada em diferentes tamanhos de tela.

### Demonstração ao Vivo

Você pode ver o Image Chat em ação em nossa [página de demonstração ao vivo](https://fastcomments.com/product/image-chat).

### Próximos Passos

Agora que você tem o básico funcionando, pode personalizar a aparência e o comportamento no guia de Opções de Configuração. Confira o guia de Design Responsivo para entender como o posicionamento baseado em porcentagem funciona. Aprenda sobre estilização e suporte ao modo escuro no guia de Personalização. Para integrações avançadas, explore a Referência da API.

### Bibliotecas de Frontend

Todas as bibliotecas de frontend do FastComments (react, vue, angular, etc) possuem o Image Chat.