Cada agente é executado contra um dos dois modelos LLM, escolhido na seção **Modelo** do formulário de edição.

### As duas opções

- **GLM 5.1 (DeepInfra) - Smarter, bit slower** - padrão. Qualidade de raciocínio mais alta, um pouco mais lenta por chamada. Recomendado para agentes no estilo de moderação (`Moderator` modelo, qualquer coisa que chame `ban_user` ou `mark_comment_spam`) onde o custo de uma chamada incorreta é alto.

- **GPT-OSS 120B Turbo (DeepInfra) - Faster** - mais rápido por chamada, menor latência. Recomendado para agentes de alto volume e baixo risco (recepcionista de boas-vindas, fixador de tópicos) onde você quer respostas em segundos e as consequências de uma chamada incorreta são pequenas.

Ambos os modelos suportam chamada de função, ambos são executados através da mesma API compatível com OpenAI, e ambos compartilham os mesmos esquemas por ferramenta - portanto você pode alternar um agente salvo entre eles a qualquer momento sem outras alterações de configuração.

### Diferenças de custo

Os dois modelos têm custos por token diferentes. Os [limites de orçamento](#budgets-overview) do agente são denominados na moeda da sua conta, não em tokens, então a troca de um modelo para outro altera quantas execuções cabem nos seus limites diários e mensais. A página de [Histórico de Execuções](#run-history) mostra o custo por execução na sua moeda assim que uma execução é concluída - observar algumas execuções após uma troca é a maneira mais fácil de avaliar a nova taxa de consumo.

### Tokens por execução

O uso de tokens da resposta do modelo é registrado em cada gatilho como **tokensUsed**. Ele é incluído nos payloads de webhook `trigger.succeeded` e `trigger.failed` (veja [Carga útil do webhook](#webhook-payloads)) e mostrado na [Visualização de Detalhes da Execução](#run-detail-view). A quantidade depende de:

- Quanto [Contexto](#context-options) você incluir - contexto do tópico, histórico do usuário e metadados da página adicionam tokens.
- O quão longos são seu [prompt inicial](#personality-prompt) e as [diretrizes da comunidade](#community-guidelines).
- Quantas ferramentas o agente chama em uma única execução (cada chamada de ferramenta e seu resultado fazem a ida e volta através do modelo).

**Max Tokens Per Trigger** (padrão 20.000) é o limite superior por execução, definido por agente.

### Alternando modelos

Você pode alternar modelos no formulário de edição a qualquer momento. O histórico de execuções e as análises existentes mantêm seus números originais de tokens e custos - eles são registrados no momento da execução. O novo modelo só se aplica às execuções que começam após você salvar.

Não existe a opção 'usar o modelo mais barato'. A escolha é explícita por agente.