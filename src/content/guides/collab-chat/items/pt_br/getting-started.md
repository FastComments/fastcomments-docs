### Início Rápido

Começar com o Collab Chat é simples. Você precisa do script FastComments Collab Chat, de um elemento HTML contendo o texto que você deseja anotar, e de um objeto de configuração com seu Tenant ID.

### Instalação

Adicione o script do Collab Chat à sua página:

[inline-code-attrs-start title = 'Carregando o script do Collab Chat'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>
[inline-code-end]

### Implementação Básica

Aqui está um exemplo mínimo:

[inline-code-attrs-start title = 'Implementação Básica do Collab Chat'; type = 'html'; isFunctional = true; inline-code-attrs-end]
[inline-code-start]
<!DOCTYPE html>
<html>
<head>
    <title>My Article with Collab Chat</title>
</head>
<body>
    <!-- Seu contêiner de conteúdo -->
    <div id="article-content" style="min-height: 500px;">
        <h1>My Article Title</h1>
        <p>This is a paragraph that users can annotate. Simply highlight any text to start a discussion!</p>
        <p>You can have multiple paragraphs, and users can highlight text across any of them.</p>
    </div>

    <!-- Load the Collab Chat script -->
    <script src="https://cdn.fastcomments.com/js/embed-collab-chat.min.js"></script>

    <!-- Inicialize o Collab Chat -->
    <script>
        FastCommentsCollabChat(document.getElementById('article-content'), {
            tenantId: 'demo'
        });
    </script>
</body>
</html>
[inline-code-end]

Replace `'demo'` with your actual FastComments Tenant ID if it's not already, which you can find in your [FastComments dashboard](https://fastcomments.com/auth/my-account/api-secret).

### Como Funciona

Uma vez inicializado, os usuários podem selecionar qualquer texto dentro do elemento alvo. Após um breve atraso (3,5 segundos no desktop), um prompt aparece permitindo que iniciem uma discussão. Quando uma discussão é criada, um destaque visual aparece no texto. Outros usuários podem passar o cursor sobre o destaque ou clicar nele para visualizar e participar da discussão. Todas as discussões sincronizam em tempo real entre todos os visitantes.

### Demo ao Vivo

Você pode ver o Collab Chat em ação em nossa [página de demonstração ao vivo](https://fastcomments.com/product/collab-chat).

### Próximos Passos

Agora que você tem o básico funcionando, você pode personalizar a aparência e o comportamento no guia de Opções de Configuração. Consulte o guia de Comportamento de Seleção de Texto para entender como a seleção de texto funciona. Saiba sobre estilização e suporte ao modo escuro no guia de Personalização. Para integrações avançadas, explore a Referência da API.

### Bibliotecas Frontend

Todas as bibliotecas frontend do FastComments (react, vue, angular, etc) incluem o Collab Chat.

---