Agent memory é um pool de pares chave-valor com escopo de tenant, **compartilhado**, que todo agente no seu tenant pode ler e escrever. Ele existe para que agentes possam carregar contexto entre execuções.

### Por que a memória existe

O contexto do LLM é por execução. Sem memória, um agente que emite um aviso a um usuário não tem como saber sobre esse aviso na próxima vez que vir o mesmo usuário. A política de escalonamento da plataforma — "advertir antes de banir" — depende do agente ser capaz de encontrar o aviso anterior. A memória é o que faz isso funcionar.

### Dois tipos de memória

- **WARNING** - escrito automaticamente como parte do fluxo [`warn_user`](#tool-warn-user). O agente não escreve registros WARNING manualmente; eles são um efeito colateral de avisar um usuário.
- **NOTE** - escrito por [`save_memory`](#tools-overview). Contexto de uso geral que o agente quer que futuros agentes saibam.

A política de escalonamento procura especificamente por registros `WARNING` ao decidir se um banimento é justificado.

### Escopo por tenant, compartilhado entre agentes

Todos os agentes no seu tenant compartilham **um único pool de memória**. Uma nota salva pelo Agente A é visível para as chamadas `search_memory` do Agente B. Isso é intencional — você quer que as notas de um agente de triagem informem as decisões de um agente moderador.

`tenantId` é definido pelo executor a partir do próprio tenant do agente — nunca pelos argumentos do LLM — então vazamentos de memória entre tenants são impossíveis por construção.

### O que há em um registro de memória

Cada entrada de memória contém:

- **Qual agente a escreveu**, e quando.
- **Sobre quem ela é** - o usuário que esta memória descreve. O agente não pode fabricar isso; a plataforma preenche automaticamente a partir do que acionou o agente.
- **Um sinal oculto de conta alternativa** - a plataforma também registra (privadamente) a impressão digital de IP do comentário de origem, para que buscas futuras na memória possam resurfacer notas sobre outras contas postando do mesmo IP. A impressão digital nunca é mostrada ao agente ou ao LLM.
- **A própria nota** - até 2000 caracteres de texto livre.
- **Tags** para recuperação - até 10 tags curtas.
- **Um tipo** - ou um warning ou uma nota geral.
- **Um link opcional para o comentário** - se a memória estiver vinculada a um comentário específico.

### Comportamento de busca

[`search_memory`](#tools-overview) retorna até 25 registros, ordenados do mais novo para o mais antigo, com escopo automaticamente definido para (o usuário do gatilho) OU (outras contas no IP do gatilho). Os resultados também são limitados em caracteres a 8000 caracteres totais através de todo o conteúdo retornado - entradas mais antigas são descartadas se o limite for atingido.

O agente não passa `userId` ou `targetIpHash`. Ambos são definidos pelo executor.

### Persistência

A memória não tem **TTL**. Os registros persistem até serem removidos explicitamente. Registros WARNING sobre um usuário são intencionalmente nunca excluídos automaticamente - o histórico de escalonamento precisa ser encontrável indefinidamente ou a verificação da plataforma de "buscar antes de banir" não faz sentido.

As três maneiras de remover memória:

- Um moderador exclui o comentário subjacente - qualquer memória vinculada a esse comentário é em cascata removida.
- Um usuário é excluído - todas as entradas de memória sobre esse usuário são removidas na mesma transação.
- Seu tenant é excluído.

Hoje não existe uma interface administrativa para excluir registros individuais de memória.

### Memória em dry-run

Agentes em dry-run não gravam memória. Isso é por design: as decisões hipotéticas de um agente em dry-run não devem poluir o pool de memória compartilhado. A leitura via `search_memory` funciona normalmente em dry-run - o agente pode ver memórias reais de agentes em produção - ele apenas não pode adicionar a elas.

### Memória em replays

Igual ao dry-run: agentes de replay não escrevem memória. Replays são apenas de pré-visualização. Veja [Test Runs (Replays)](#test-runs-replays).

### Resumo das restrições

| Limite | Valor |
|---|---|
| Tamanho máximo do conteúdo da memória | 2000 caracteres |
| Tamanho máximo de uma tag de memória | 64 caracteres |
| Máximo de tags de memória | 10 |
| Tamanho máximo da consulta de memória | 200 caracteres |
| Limite de resultados da busca de memória | 25 registros |
| Limite total de conteúdo da busca de memória | 8000 caracteres |

### Veja também

- [Ferramenta: save_memory](#tools-overview) para escrita.
- [Ferramenta: search_memory](#tools-overview) para leitura.
- [Ferramenta: warn_user](#tool-warn-user) - a única ferramenta que escreve memória do tipo WARNING.
- [Ferramenta: ban_user](#tool-ban-user) - o prompt do sistema exige que `search_memory` seja chamado antes disso.