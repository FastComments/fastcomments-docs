FastComments aplica o Artigo 17 do Digital Services Act (DSA) da UE para tenants na região da UE: **suspensões de usuário totalmente automatizadas não são permitidas**.

### O que isso significa na prática

Quando seu tenant está na região da UE, no formulário de edição do agente:

- A caixa de seleção **Approvals** para `ban_user` está **bloqueada como ativada** e não pode ser desmarcada.
- O rótulo diz: "Artigo 17 do DSA da UE: suspensões de usuários exigem revisão humana. 'Ban a user' está bloqueado como ativado e não pode ser totalmente automatizado na região da UE."
- Uma tooltip na coluna de aprovação diz: "Bloqueado pelo Artigo 17 do DSA da UE - bans totalmente automatizados não são permitidos na região da UE."

Independentemente do que mais você configurar, toda chamada `ban_user` de qualquer agente em um tenant na região da UE vai para a [caixa de entrada de aprovações](#approval-workflow) para revisão humana. O ban não ocorre até que um humano o aprove.

### Por que isso é imposto no nível da plataforma, e não no nível do prompt

Prompts do sistema podem ser ignorados ou contornados por um modelo suficientemente mal comportado. A conformidade com o Artigo 17 é importante demais para depender do "bom comportamento" do modelo; precisa ser um bloqueio rígido do lado do servidor que o próprio despachante de ferramentas aplica. E é isso que fazemos.

### O que passa e o que não passa por aprovação

- **`ban_user`**: sempre bloqueado na UE. Inclui:
  - Bans visíveis (`shadowBan: false`).
  - Shadow bans (`shadowBan: true`).
  - Bans com `deleteAllUsersComments: true`.
  - Bans com `banIP: true`.
- Todas as variantes de ban chegam na caixa de entrada de aprovações com o raciocínio e a confiança do agente; um humano aprova ou rejeita.

As outras ferramentas do agente (`mark_comment_spam`, `warn_user`, `lock_comment`, etc.) **não** são afetadas pelo Artigo 17. Você ainda pode automatizá-las. O Artigo 17 trata especificamente de suspensões de usuários.

### E os tenants fora da UE

O bloqueio não se aplica fora da região da UE. Você pode optar por exigir aprovação para `ban_user` mesmo assim - recomendamos fortemente fazê-lo nas primeiras semanas de vida de qualquer agente de moderação - mas não é imposto.

### Shadow bans

Shadow bans contam como suspensões para fins do Artigo 17 (o usuário pode postar, mas o conteúdo dele fica oculto). Eles são bloqueados da mesma forma que bans visíveis.

### Detecção de região

A região é determinada a nível de processo pela variável de ambiente `REGION` na implantação do FastComments (lida por `isEURegion()` em `models/constants.ts`). Não existe um campo de região por tenant - o bloqueio se aplica a todo tenant em uma instância implantada na UE. Se você migrar seus dados de uma implantação fora da UE para uma implantação na UE, o bloqueio entra em vigor para todos os tenants dessa instância.

### E se todos os revisores estiverem indisponíveis

A aprovação ficará na caixa de entrada até ser decidida. Ela expira automaticamente 90 dias após a criação. Não existe um caminho "sem revisor disponível, passar para decisão automatizada" - isso derrotaria o propósito do Artigo 17.

Se sua comunidade for tão grande que bans na UE não possam ser revisados em tempo razoável, considere:

- Adicionar mais revisores (veja [Notificações de Aprovação](#approval-notifications)).
- Alterar o agente para usar [`warn_user`](#tool-warn-user) com mais agressividade, já que avisos não estão sujeitos ao Artigo 17.
- Reduzir a propensão do agente a banir, apertando as [diretrizes da comunidade](#community-guidelines) ou o [prompt inicial](#personality-prompt).

### Veja também

- [Ferramenta: ban_user](#tool-ban-user) para o que `ban_user` faz e as opções destrutivas por trás de opt-ins extras.
- [Fluxo de Aprovação](#approval-workflow) para o ciclo completo de aprovação.