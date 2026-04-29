A ferramenta de banimento é a ação mais consequente que um agente pode executar. Ela banea um usuário da sua comunidade, com uma duração fixa e algumas opções.

### O que ela faz

O agente escolhe uma das seis durações:

- Uma hora
- Um dia
- Uma semana
- Um mês
- Seis meses
- Um ano

Ele também escolhe entre um **banimento visível** (o usuário vê uma mensagem clara de banimento e pode apelar) e um **shadow ban (banimento oculto)** (o usuário pode continuar postando, mas seu conteúdo fica oculto para outros usuários). As instruções da plataforma orientam o agente a preferir banimentos visíveis para casos de primeira vez ou limítrofes, e shadow bans para infratores reincidentes claramente maliciosos.

### As duas sub-opções destrutivas

Duas opções extras são **ocultas ao agente por padrão**. Para habilitar qualquer uma delas, marque a caixa correspondente na seção **Opções de banimento** no formulário de edição do agente:

- **Allow deleting all of the user's comments.** Quando habilitada, o agente pode optar também por excluir todos os comentários que o usuário banido já postou no seu tenant. Reserve para spam claro, divulgação de dados pessoais (doxxing), ou abuso coordenado onde o conteúdo existente não tem valor. **Destrutivo e irreversível.**
- **Allow banning by IP.** Quando habilitada, o agente pode optar também por banir o IP de onde o comentário foi postado. Útil contra evasão de ban por contas alternativas. **Evite para IPs compartilhados** (empresas, escolas, operadoras móveis) — usuários inocentes na mesma rede serão bloqueados.

A plataforma também limita isso no lado do servidor: mesmo se o agente agir maliciosamente e tentar invocar a opção, a solicitação é recusada a menos que você tenha optado por ela.

### Política de escalonamento

Antes de banir, a plataforma instrui o agente a:

1. Pesquisar na [memória do agente](#agent-memory-system) por avisos ou anotações anteriores sobre o usuário.
2. Preferir [advertir](#tool-warn-user) o usuário em vez de banir por ofensas iniciais.
3. Pular a etapa de aviso apenas para casos claramente gravíssimos (conteúdo ilegal, doxxing, spam coordenado) — e explicar o porquê em sua justificativa.

Essa política está nas instruções do agente, não é uma regra rígida do servidor, por isso **recomenda-se fortemente exigir aprovação para banimentos**.

### Região UE: aprovação humana necessária

Na região da UE, esta ferramenta é **bloqueada para aprovação** pelo Artigo 17 da Lei de Serviços Digitais. Todo ban de qualquer agente em um tenant da região da UE cai na [caixa de aprovações](#approval-workflow) para revisão humana. Veja [Conformidade com o Artigo 17 da DSA da UE](#eu-dsa-compliance).

### Recomendações

- Exija aprovação em todos os lugares por pelo menos o primeiro mês.
- Sempre exija aprovação para a opção **delete-all-comments** se você a habilitar — ela é irreversível.
- Considere exigir aprovação para a opção **IP ban** mesmo depois que o agente ganhar confiança — o custo de um banimento por IP em uma rede compartilhada não aparece no histórico de execuções do agente.

### Veja também

- [Banning Users](/guide-moderation.html#banning-users) e [Banning Users With Wildcards](/guide-moderation.html#banning-users-wildcards) no guia de moderação para como os banimentos funcionam em toda a plataforma.
- [Advertir usuário](#tool-warn-user) — a etapa de escalonamento mais branda.