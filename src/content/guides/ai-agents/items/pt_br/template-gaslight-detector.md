**ID do modelo:** `gaslight_detector`

O Detector de Gaslighting monitora edições de comentários que reescrevem o histórico no meio de uma conversa - o tipo em que um autor altera o significado de um comentário anterior depois que as respostas já foram escritas, deixando as respostas subsequentes parecendo fora de contexto ou erradas. Quando o agente decide que uma edição cruza essa linha, ele restaura o texto original e envia uma DM ao autor para explicar.

Este é um modelo de risco mais elevado porque modifica conteúdo de usuários. Execute-o em [dry-run](#dry-run-mode) por mais tempo do que executaria um modelo somente de leitura, e restrinja `edit_comment` atrás de [aprovação](#approval-workflow) até confiar no julgamento do modelo com o seu tráfego.

### Gatilhos

- **Comentário editado** (`COMMENT_EDIT`) - o agente compara o texto novo e o anterior e decide se a edição distorce as respostas que já existem.

Veja [Gatilho: Comentário Editado](#trigger-comment-edit) para o payload completo, incluindo o texto anterior do comentário e a contagem de respostas no momento da edição.

### Ferramentas permitidas

- [`edit_comment`](#tool-edit-comment) - usada para restaurar o texto original quando a edição é considerada gaslighting.
- [`warn_user`](#tool-warn-user) - emite um aviso suave que o usuário vê em sua próxima visita.
- [`send_dm`](#tools-overview) - o canal de explicação; o usuário recebe uma mensagem direta descrevendo por que sua edição foi revertida.

Ele não pode banir, marcar como spam, votar ou postar novos comentários - a superfície é intencionalmente limitada.

### Adições recomendadas antes de entrar em produção

- **Restrinja `edit_comment` atrás de [aprovação](#approval-workflow).** Reverter um comentário é visível para o autor e para qualquer pessoa que viu a versão editada, então um falso positivo é embaraçoso. Mantenha as aprovações ativadas até que o dry-run mostre que o agente é consistente.
- **Refine o prompt com o que conta como gaslighting no seu site.** O prompt padrão é curto de propósito. Dê ao modelo exemplos concretos - "inverter uma afirmação de sim/não", "deletar um número que as respostas citam", "adicionar uma frase hostil depois que as respostas foram postadas" - e exemplos explícitos do que não conta, como correções de erros de digitação, limpeza de formatação ou adição de fontes.
- **Use a contagem de respostas do contexto do gatilho.** Edições em comentários sem respostas não podem distorcer uma conversa; o prompt deve instruir o modelo a ignorá-las.
- **Marque "Include commenter's trust factor, account age, ban history, and recent comments"** em [Opções de Contexto](#context-options). O modelo é muito menos agressivo quando pode ver uma conta antiga e de boa-fé.
- **Considere uma janela curta de tolerância para edições no prompt.** Muitas edições nos primeiros 30-60 segundos são correções de erros de digitação; instrua o modelo a ignorar edições tão rápidas.

### Janela de dry-run recomendada

Execute por pelo menos duas semanas de tráfego real em [dry-run](#dry-run-mode) antes de mudar para Enabled, e revise todas as edições sinalizadas durante essa janela. Use [Execuções de Teste (Replays)](#test-runs-replays) para reproduzir os últimos 30 dias de edições contra o agente antes de entrar em produção.