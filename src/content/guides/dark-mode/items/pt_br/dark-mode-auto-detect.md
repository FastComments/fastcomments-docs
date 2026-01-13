Por padrão, o FastComments detectará automaticamente se o seu site tem um fundo escuro com base na "distância do preto" no círculo de cores.

Nossos produtos fazem o melhor possível nisso, no entanto, há quase infinitas cores na roda de cores, e pode haver cenários onde a aplicação escolhe usar o modo escuro quando não é apropriado, e vice-versa. Esta documentação cobre como ter um controle mais refinado sobre isso.

#### Detalhes técnicos

Detectamos o modo escuro percorrendo os elementos na página para cima a partir do widget de comentários, procurando um fundo escuro quando o widget é carregado inicialmente.

Para alternar o modo escuro após esta etapa, você deve chamar o widget para atualizar sua configuração. Isso é coberto na seção `Configuração manual`.
