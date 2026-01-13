Para o plugin funcionar, um token é salvo no banco de dados do seu WordPress e também na sua conta FastComments. Quando o plugin faz solicitações aos nossos servidores, ele fornece
este token.

Você pode ver todas as integrações autorizadas na sua conta FastComments [aqui](https://fastcomments.com/auth/my-account/manage-data/integrations).

Toda a comunicação é feita via HTTPS.

Toda a comunicação é *de saída* do seu servidor WordPress *para* o FastComments.com, incluindo a sincronização *de volta* para sua instalação do WordPress, pois ela é implementada via [polling](https://en.wikipedia.org/wiki/Polling_(computer_science)) a partir de uma configuração de [cron](https://developer.wordpress.org/plugins/cron/) na sua instalação do WordPress.