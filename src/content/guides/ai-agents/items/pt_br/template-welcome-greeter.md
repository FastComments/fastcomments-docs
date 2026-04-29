**Template ID:** `welcome_greeter`

O Saudador de Boas-Vindas responde calorosamente a quem comenta pela primeira vez. É o modelo de menor risco (sem ferramentas destrutivas) e um bom primeiro agente para colocar em produção.

### Gatilhos

- **Novo usuário publica seu primeiro comentário neste site** (`NEW_USER_FIRST_COMMENT`).

Este evento é acionado exatamente uma vez por usuário, portanto o agente não pode entrar em loop. Veja [Gatilho: Primeiro comentário de novo usuário](#trigger-new-user-first-comment).

### Ferramentas permitidas

- [`write_comment`](#tools-overview)

Essa é a única ferramenta - o agente literalmente não pode moderar, votar, banir, ou enviar mensagens diretas (DM).

### Recomendações antes de entrar em produção

- **Defina o nome exibido** para algo convidativo - "Community Bot", seu mascote do site, ou o nome da sua marca. O nome exibido é o que os leitores veem anexado à resposta de boas-vindas.
- **Marque "Incluir título da página, subtítulo, descrição e meta tags"** em [Opções de Contexto](#context-options). As respostas do saudador melhoram visivelmente quando ele pode fazer referência ao conteúdo da página.
- **Considere restrições de localidade** se você opera em múltiplos idiomas. Uma resposta de boas-vindas no idioma errado é mais desconcertante do que uma resposta perdida. Veja [Escopo: Filtros de URL e Localidade](#scope-url-locale).

### Por que nenhuma aprovação é necessária

O agente apenas escreve novos comentários e somente em um gatilho único. No pior cenário: uma saudação constrangedora. Não há ação destrutiva a ser controlada. A maioria dos operadores roda este sem aprovações assim que a execução de teste (dry-run) estiver limpa.