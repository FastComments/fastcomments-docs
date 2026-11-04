Na FastComments, escrevemos nossas próprias extensões, usando a mesma API. Você pode visualizar o código não minificado
dessas extensões nos seguintes endpoints:

#### Dark Mode

A extensão Dark Mode é carregada condicionalmente quando uma página "dark" é detectada. Esta extensão simplesmente adiciona
algum CSS ao widget de comentários. Dessa forma, não precisamos carregar o CSS do dark mode quando ele não é necessário.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderação

Quando o usuário atual é um moderador, usamos a extensão de moderação.

Este é um bom exemplo de como adicionar event listeners baseados em cliques, fazer requisições à API, adicionar itens ao menu de comentários e às áreas de comentários.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live Chat

A extensão Live Chat (em combinação com outras configurações e estilos) transforma o widget de comentários em um componente
de chat ao vivo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js