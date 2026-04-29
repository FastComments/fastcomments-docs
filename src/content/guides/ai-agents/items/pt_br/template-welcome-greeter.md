**ID do Modelo:** `welcome_greeter`

O Saudador de Boas-Vindas responde calorosamente a comentaristas de primeira viagem. É o template de menor risco (sem ferramentas destrutivas) e um bom primeiro agente para colocar em produção.

### Prompt inicial embutido

[inline-code-attrs-start title = 'Prompt inicial do template Saudador de Boas-Vindas'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a warm community greeter. Reply to first-time commenters with a short, personal welcome. Mention one specific thing from their comment so it does not read as a template. Keep replies to 1-2 sentences. Never reply to accounts more than 24 hours old.
[inline-code-end]

### Gatilhos

- **Novo usuário publica seu primeiro comentário neste site** (`NEW_USER_FIRST_COMMENT`).

Esse evento é acionado exatamente uma vez por usuário, então o agente não pode entrar em loop. Veja [Gatilho: New User First Comment](#trigger-new-user-first-comment).

### Ferramentas permitidas

- [`write_comment`](#tools-overview)

Essa é a única ferramenta - o agente literalmente não pode moderar, votar, banir ou enviar mensagens diretas (DM).

### Adições recomendadas antes de entrar em produção

- **Defina o nome exibido (Display name)** para algo convidativo - "Community Bot", o mascote do seu site, ou o nome da sua marca. O nome exibido é o que os leitores veem anexado à resposta de boas-vindas.
- **Marque "Incluir título da página, subtítulo, descrição e meta tags"** em [Opções de Contexto](#context-options). As respostas do saudador melhoram visivelmente quando ele pode referenciar sobre o que a página realmente trata.
- **Considere restrições de localidade** se você opera em múltiplos idiomas. Uma resposta de boas-vindas no idioma errado é mais chocante do que uma resposta perdida. Veja [Escopo: Filtros de URL e Localidade](#scope-url-locale).

### Por que não são necessárias aprovações

O agente apenas escreve novos comentários e somente com um gatilho único. No pior cenário: uma saudação constrangedora. Não há ação destrutiva a ser controlada. A maioria dos operadores roda este sem aprovações depois que o dry-run parecer limpo.