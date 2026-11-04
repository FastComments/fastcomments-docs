O FastComments aplica o Artigo 17 do Ato de Serviços Digitais da UE para tenants na região da UE: **suspensões de usuários totalmente automatizadas não são permitidas**.

### O que isso significa na prática

Quando seu tenant está na região da UE, no formulário de edição do agente:

- A caixa de seleção **Approvals** para `ban_user` está **travada ativada** e não pode ser desmarcada.
- O rótulo diz: "EU DSA Article 17: user suspensions require human review. 'Ban a user' is locked on and cannot be fully automated in the EU region."
- Um tooltip na coluna de aprovação diz: "Locked on by EU DSA Article 17 - fully-automated bans are not permitted in the EU region."

Independentemente do que mais você configurar, toda chamada de `ban_user` de qualquer agente em um tenant da região da UE vai para a [caixa de entrada de aprovações](#approval-workflow) para revisão humana. O banimento não acontece até que um humano o aprove.

### Por que isso é aplicado no nível da plataforma, e não no nível do prompt

Prompts de sistema podem ser ignorados ou contornados por um modelo que se comporte mal o suficiente. A conformidade com o Artigo 17 é importante demais para depender do bom comportamento do modelo; precisa ser um bloqueio rígido no lado do servidor que o próprio despachante de ferramentas aplica. E é o que fazemos.

### O que passa e o que não passa por aprovação

- **`ban_user`**: sempre sujeito a aprovação na UE. Incluindo:
  - Banimentos visíveis (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Banimentos com `deleteAllUsersComments: true`.
  - Banimentos com `banIP: true`.
- Todas as variantes de banimento chegam à caixa de entrada de aprovações com o raciocínio e a confiança do agente; um humano aprova ou rejeita.

As outras ferramentas do agente (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **não** são afetadas pelo Artigo 17. Você ainda pode automatizá-las. O Artigo 17 trata especificamente de suspensões de usuários.

### E quanto aos tenants fora da UE

O bloqueio não se aplica fora da região da UE. Você pode optar por colocar `ban_user` atrás de aprovação mesmo assim - recomendamos fortemente isso nas primeiras semanas de vida de qualquer agente de moderação - mas não é obrigatório.

### Shadow bans

Shadow bans contam como suspensões para fins do Artigo 17 (o usuário pode postar, mas seu conteúdo fica oculto). Eles são submetidos à aprovação de forma idêntica aos banimentos visíveis.

### Detecção de região

A região é determinada no nível do processo pela variável de ambiente `REGION` na implantação do FastComments (lida por `isEURegion()` em `models/constants.ts`). Não há campo de região por tenant - o bloqueio se aplica a todos os tenants em uma instância implantada na UE. Se você migrar seus dados de uma implantação fora da UE para uma implantação na UE, o bloqueio entra em vigor para todos os tenants dessa instância.

### E se todos os revisores estiverem indisponíveis

A aprovação ficará na caixa de entrada até ser decidida. Ela expira automaticamente 90 dias após a criação. Não existe um caminho de "nenhum revisor disponível, recorrer à decisão automatizada" - isso anularia o propósito do Artigo 17.

Se sua comunidade tem um volume tão alto que os banimentos na UE não podem ser revisados em um tempo razoável, considere:

- Adicionar mais revisores (veja [Notificações de Aprovação](#approval-notifications)).
- Fazer o agente usar [`warn_user`](#tool-warn-user) com mais frequência, já que avisos não estão sujeitos ao Artigo 17.
- Reduzir a propensão do agente a banir, restringindo as [diretrizes da comunidade](#community-guidelines) ou o [prompt inicial](#personality-prompt).

### Veja também

- [Ferramenta: ban_user](#tool-ban-user) para o que `ban_user` faz e as opções destrutivas atrás de opt-ins adicionais.
- [Fluxo de Aprovação](#approval-workflow) para o ciclo de vida completo da aprovação.