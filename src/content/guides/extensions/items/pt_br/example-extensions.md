Na FastComments, escrevemos nossas próprias extensões, usando a mesma API. Você pode visualizar o código não-minificado para essas extensões nos seguintes endpoints:

#### Dark Mode

A extensão Dark Mode é carregada condicionalmente quando uma página "dark" é detectada. Esta extensão simplesmente adiciona algum CSS ao widget de comentários. Dessa forma não precisamos carregar o CSS do modo escuro quando não é necessário.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.dark.extension.js

#### Moderation

Quando o usuário atual é um moderador, usamos a extensão de moderação.

Este é um bom exemplo para adicionar manipuladores de eventos baseados em clique, fazer requisições à API, adicionar ao menu do comentário e às áreas de comentário.

https://fastcomments.com/js/comment-ui/extensions/comment-ui.moderation.extension.js

#### Live Chat

A extensão Live Chat (em combinação com outras configurações e estilos) transforma o widget de comentários em um componente de chat ao vivo.

https://fastcomments.com/js/comment-ui/extensions/live-chat.extension.js