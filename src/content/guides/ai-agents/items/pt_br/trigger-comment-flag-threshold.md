Dispara quando a contagem de denúncias de um comentário atinge **exatamente** o limite configurado.

### Configuração necessária

- **Flag threshold** - inteiro >= 1. O gatilho dispara no momento em que `flagCount === flagThreshold`. Ele não dispara novamente em sinalizações subsequentes além do limite.

Se o limite for 3 e três usuários sinalizarem o comentário, o agente dispara uma vez na terceira sinalização. Um quarto, quinto ou sexto sinal **não** o fará disparar novamente.

### Contexto que o agente recebe

- O comentário sinalizado.
- Histórico opcional do tópico / usuário / contexto da página conforme configurado.
- A contagem de denúncias está no bloco do comentário como `Flag Count: N`.

### Observações

- O gatilho só dispara quando o comentário cruza o limite por baixo através do caminho de tratamento de flags da plataforma (onde `didIncrement === true`). Gravações diretas no DB que definem `flagCount` para o valor do limite não o disparam; sinalizações além do limite também não o disparam novamente.
- Não inclui quem sinalizou o comentário - as sinalizações são anônimas para o agente. Se você quiser verificar os usuários que sinalizaram, busque-os em seus próprios dados.
- Um atraso do gatilho (veja [Gatilhos Diferidos](#trigger-deferred-delay)) é *fortemente* recomendado para este gatilho - sinalizações frequentemente chegam em rajadas durante um tópico acalorado, e um pequeno atraso permite que a situação se estabilize antes de o agente agir.

### Usos comuns

- **Revisão de moderação** - um comentário sinalizado é o sinal canônico de "os humanos acham que isso pode ser ruim". O [Modelo de Moderador](#template-moderator) se inscreve nesse gatilho por padrão com um limite de denúncias de 3.
- **Aumento da fila de pré-moderação** - o agente executa uma passagem inicial e ou marca o comentário para moderação (com `mark_comment_reviewed`) ou escala para revisão adicional.
- **Contra brigadas** - combine este gatilho com [contexto de histórico do usuário](#context-options) e permita que o agente veja banimentos anteriores/sinais de conteúdo duplicado antes de agir.

### Recomendações de combinação

Inscreva-se em **ambos** `COMMENT_ADD` e `COMMENT_FLAG_THRESHOLD` se você quiser um agente de moderação que capture casos óbvios à primeira vista e reavalie casos limítrofes uma vez que as sinalizações se acumulem. Os dois eventos disparam independentemente - o agente será executado duas vezes se ambos estiverem inscritos e ambos dispararem, mas a segunda execução verá o estado agora sinalizado.

---