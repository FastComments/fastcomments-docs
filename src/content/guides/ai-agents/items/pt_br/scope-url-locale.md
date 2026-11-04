Por padrão, um agente roda em todo o seu tenant - todas as páginas, todos os locales. As seções **Scope** e **Locales** no formulário de edição permitem restringir isso.

### Restringir a páginas específicas

O campo **Restrict to specific pages** aceita um padrão de URL por linha, na sintaxe glob de url-pattern. O agente só roda em comentários cuja URL da página corresponda a pelo menos um dos padrões. Exemplos:

- `/news/*` - qualquer página sob `/news`.
- `/forums/*` - qualquer página sob `/forums`.
- `/blog/2026/*` - qualquer página sob `/blog/2026`.
- (múltiplas linhas juntas) - o agente roda se **qualquer** padrão corresponder.

Máximo: 50 padrões por agente. Os padrões devem ser globs de url-pattern válidos - o formulário rejeita os malformados com um erro específico.

Quando o campo está **em branco**, o agente roda em todas as páginas do tenant.

Quando o campo está **preenchido**, o agente falha de forma segura (fail closed): qualquer gatilho cujo comentário não tenha `urlId` (por exemplo, eventos de nível de tenant sem contexto de página) é ignorado. Isso é intencional - "restrito a /news/*" não deve silenciosamente se estender para "tudo".

### Restringir a locales específicos

O seletor de lista dupla **Restrict to specific locales** aceita IDs de locale do FastComments (`en_us`, `zh_cn`, `de_de`, etc.). O agente só roda em comentários cujo locale detectado esteja na lista selecionada.

O locale detectado vem do campo `locale` do comentário, que é definido pelo widget de comentários no momento da postagem com base no locale da página.

Quando **nenhum locale** está selecionado, o agente roda em todos os locales.

Quando **um ou mais locales** estão selecionados, o agente falha de forma segura: gatilhos sem comentário, ou gatilhos em comentários sem campo `locale`, são ignorados.

### Escopo combinado

Os filtros de URL e locale são combinados com AND. Um gatilho só aciona o agente se **ambos** os filtros permitirem.

Padrões úteis:
- Padrão de URL `/news/*` + locale `en_us` - apenas a seção de notícias em inglês.
- Sem filtro de URL + múltiplos locales - em todo o tenant, mas apenas para os idiomas para os quais o prompt deste agente foi escrito.

### Por que restringir o escopo de um agente

- **Custo.** Restringir o escopo reduz o volume de gatilhos que o agente precisa processar e, portanto, reduz o gasto com tokens.
- **Corretude.** Um prompt de resumo ajustado para artigos técnicos pode produzir resultados ruins em páginas de produtos. Restringir o escopo é uma ferramenta mais precisa do que pedir ao prompt para "pular páginas não técnicas" em inglês.
- **Comportamento específico por locale.** Um saudador de boas-vindas que só escreve em alemão deve rodar apenas em comentários com locale alemão. Combine o escopo de locale `de_de` com um tom em alemão no [prompt inicial](#personality-prompt).

### O que a restrição de escopo *não* faz

- Ela não altera a **contagem de slots de agente** (veja [Plans and Eligibility](#plans-and-eligibility)) - um agente com escopo restrito ainda ocupa um slot.
- Ela não altera os [Limites de orçamento](#budgets-overview) - os limites diários e mensais por agente se aplicam a todos os gatilhos correspondentes.
- Ela não restringe retroativamente execuções passadas - o histórico de execuções mostra tudo o que o agente fez, mesmo que você restrinja o escopo depois.