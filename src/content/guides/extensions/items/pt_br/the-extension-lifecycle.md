O script para cada extensão é buscado e invocado antes que o widget de comentários comece a buscar o primeiro conjunto de comentários e renderizar a UI.

No carregamento inicial, os seguintes dados serão anexados ao objeto da extensão:

- `config` - Uma referência ao objeto `config`.
- `translations` - Uma referência ao objeto `translations`.
- `commentsById` - Uma referência a todos os comentários por id.
- `root` - Uma referência ao nó DOM raiz.

As extensões devem sobrescrever as funções desejadas, que o widget de comentários chamará nos momentos apropriados.