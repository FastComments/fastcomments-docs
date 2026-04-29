Por padrão, um agente é executado em todo o seu tenant - cada página, cada localidade. As seções **Scope** e **Locales** no formulário de edição permitem que você restrinja isso.

### Restrict to specific pages

O campo **Restrict to specific pages** aceita um padrão de URL por linha, na sintaxe de glob url-pattern. O agente só é executado em comentários cuja URL da página corresponda a pelo menos um dos padrões. Exemplos:

- `/news/*` - qualquer página sob `/news`.
- `/forums/*` - qualquer página sob `/forums`.
- `/blog/2026/*` - qualquer página sob `/blog/2026`.
- (várias linhas em conjunto) - o agente é executado se **qualquer** padrão corresponder.

Maximum: 50 patterns per agent. Patterns must be valid url-pattern globs - the form rejects malformed ones with a specific error.

Quando o campo está **em branco**, o agente é executado em todas as páginas do tenant.

Quando o campo está **não em branco**, o agente falha fechado: qualquer trigger cujo comentário não tenha `urlId` (por exemplo, eventos em nível de tenant sem contexto de página) é ignorado. Isso é intencional - "scoped to /news/*" não deve silenciosamente se aplicar a "everything".

### Restrict to specific locales

O seletor dual-list **Restrict to specific locales** aceita FastComments locale IDs (`en_us`, `zh_cn`, `de_de`, etc.). O agente só é executado em comentários cuja localidade detectada esteja na lista selecionada.

A localidade detectada vem do campo `locale` do comentário, que é definido pelo widget de comentário no momento do envio com base na localidade da página.

Quando **nenhuma localidade** está selecionada, o agente é executado em todas as localidades.

Quando **uma ou mais localidades** estão selecionadas, o agente falha fechado: triggers sem um comentário, ou triggers em comentários sem o campo `locale`, são ignorados.

### Combined scoping

URL and locale filters AND together. Um trigger só dispara o agente se **ambos** os filtros permitirem.

Padrões úteis:
- `/news/*` URL pattern + `en_us` locale - apenas a seção de notícias em inglês.
- Sem filtro de URL + múltiplas localidades - em todo o tenant, mas apenas para os idiomas para os quais o prompt deste agente foi escrito.

### Why scope an agent

- **Custo.** Escopo reduz o volume de triggers que o agente precisa processar, e assim reduz o gasto de tokens.
- **Correção.** Um prompt de sumarização ajustado para artigos técnicos pode produzir saída ruim em páginas de produto. Escopar é uma ferramenta mais precisa do que pedir ao prompt para "pular páginas não técnicas" em inglês.
- **Comportamento específico por localidade.** Um bot de boas-vindas que escreve apenas em alemão deve ser executado somente em comentários com localidade alemã. Combine o escopo de localidade `de_de` com um tom em alemão no [initial prompt](#personality-prompt).

### What scoping does *not* do

- Não altera a **agent slot count** (veja [Plans and Eligibility](#plans-and-eligibility)) - um agente escopado ainda ocupa um slot.
- Não altera os [Budget caps](#budgets-overview) - os limites diários e mensais por agente se aplicam a todos os triggers correspondentes.
- Não escopa retroativamente execuções passadas - o histórico de execuções mostra tudo o que o agente fez, mesmo se você apertar o escopo depois.

---