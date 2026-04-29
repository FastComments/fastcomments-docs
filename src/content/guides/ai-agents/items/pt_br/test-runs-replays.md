Uma **execução de teste** (também chamada de **replay**) executa o agente contra uma janela de comentários históricos **sem realizar ações reais**. É a forma mais rápida de visualizar o comportamento do agente antes de ir ao vivo.

Acessível a partir da página de lista de agentes através do botão **Execução de teste** na linha de cada agente.

### O que faz

A plataforma:

1. Seleciona uma amostra de comentários históricos que correspondem ao escopo do agente, na janela que você escolher.
2. Para cada comentário, executa o agente de ponta a ponta como se o comentário tivesse acabado de ser postado - mesmo contexto, mesma chamada de LLM, mesma seleção de ferramentas, mesmas justificativas e pontuações de confiança.
3. Registra cada execução como uma execução simulada (dry-run), etiquetada para que permaneça agrupada com o replay de onde veio e excluída das visualizações de execuções ao vivo.
4. **Compara** o veredito do agente com o que realmente aconteceu ao comentário - ele foi depois aprovado, marcado como spam, excluído, bloqueado pelo motor de spam, etc.

O resultado é um diff por comentário: "O agente em replay marcaria isto como spam, mas o comentário está atualmente aprovado e limpo."

### Configuração

A página de execução de teste tem um único campo de entrada:

- **Dias de comentários históricos a serem avaliados** - um campo numérico `days` entre 1 e 90. Comentários mais antigos não são elegíveis.

O tamanho da amostra e o limite máximo não são expostos na interface do usuário - ambos são padrões aplicados no servidor por plano. A página mostra campos informativos:

- **Comentários que correspondem na janela** - quantos comentários seriam considerados.
- **Até N comentários desta janela serão processados** - o tamanho efetivo da amostra dado o limite do lado servidor.
- **Custo estimado** - na moeda do seu tenant.

### Limite de taxa

Cada usuário está limitado a **10 execuções de teste por 24 horas** (limitado por taxa via a chave `replay-create:${requestedBy}`). O botão mostra uma tooltip quando você atingiu o limite ("Você atingiu 10 execuções de teste nas últimas 24 horas.").

### Concorrência

Apenas um replay pode estar ativo por agente ao mesmo tempo. Iniciar um segundo replay enquanto um está em andamento redireciona você para o replay em voo.

### Leitura de resultados

Quando o replay termina, a página de resultados mostra abas:

- **Deltas** (ativa por padrão) - o veredito do agente no replay difere da realidade. (O mais interessante - "o agente teria marcado este comentário como spam, mas o comentário foi aprovado e está ok".)
- **Matches** - o veredito do agente no replay corresponde ao que realmente aconteceu. (Reassurance - o agente concorda com a realidade.)
- **Sem ação** - o agente do replay decidiu não fazer nada. (Às vezes a resposta correta; às vezes o agente deixou passar algo.)
- **Todos** - todo resultado independentemente da classificação.

Para cada comentário em qualquer aba:

- **Resultado prévio** - a classificação do que realmente aconteceu: **POSITIVE**, **NEGATIVE**, ou **INDETERMINATE**, com **Evidência** ("Comment marked deleted at {date}", "Engine: bayes", e assim por diante).
- **O agente em replay faria** - a ação escolhida pelo agente.
- **Por quê** - a justificativa.
- **Confiança** - exibida como porcentagem.

### Por que os replays forçam execução simulada

Um replay contra um comentário que foi excluído há quatro meses não deve excluí-lo retroativamente - ele já está excluído. Um replay contra um comentário que o agente agora quer aprovar não deve alterar o estado atual do comentário. Replay é uma ferramenta de pré-visualização. Forçar execução simulada é o que torna seguro executar um replay contra qualquer janela histórica.

### Reprodutibilidade

Replays congelam a configuração do agente no momento em que o replay foi iniciado. Edições subsequentes ao agente não alteram os resultados do replay - a página de resultados permanece estável como um registro do que *essa* versão do agente teria feito.

### Quando orçamentos param um replay

Replays estão sujeitos a:

- Seu próprio **limite máximo** (definido no formulário de replay).
- Os limites diários e mensais de **orçamento** do agente.
- Os limites diários e mensais de **orçamento** do tenant.

O primeiro deles que for atingido aborta o replay com um código de erro específico. Quaisquer resultados por comentário produzidos antes do aborto são preservados em [Histórico de execuções](#run-history).

### Como os replays são executados

Replays são executados em segundo plano, não de forma síncrona. Depois de clicar em "Iniciar execução de teste", o replay é enfileirado e um worker o pega. Um replay longo pode durar vários minutos. A página de resultados faz polling e mostra o progresso (contagem processada, gasto até agora) conforme avança.

Se um worker morrer no meio do replay, a plataforma re-enfileira automaticamente o replay para que ele retome na próxima passada. Uma breve interrupção nunca deixa um replay órfão.

### O que o replay não faz

- **Não respeita [trigger delays](#trigger-deferred-delay).** Replays são executados imediatamente, não 30 minutos depois.
- **Não escreve na memória.** Agentes em replay não salvam notas de memória, mesmo que sua lógica normalmente o fizesse.
- **Não dispara webhooks.** Gatilhos produzidos pelo replay não geram eventos de webhook `trigger.succeeded` / `trigger.failed`.
- **Não exclui comentários já reproduzidos.** Executar um segundo replay contra a mesma janela cobre os mesmos comentários.

### Veja também

- [Refining Prompts](#refining-prompts) - o fluxo de trabalho de edição iterativa que combina bem com replays.
- [Modo de execução simulada](#dry-run-mode) - a mesma ideia, contra tráfego ao vivo.