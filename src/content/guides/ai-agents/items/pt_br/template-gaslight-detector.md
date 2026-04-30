**Template ID:** `gaslight_detector`

O Detector de Gaslighting observa edições de comentários que reescrevem a história no meio de uma conversa — do tipo em que um autor altera o significado de um comentário anterior depois que respostas já foram escritas, deixando respostas posteriores fora de contexto ou erradas. Quando o agente decide que uma edição ultrapassa esse limite, ele restaura o texto original e envia uma mensagem direta ao autor para explicar.

Este é um template de risco mais alto porque modifica conteúdo de usuários. Execute-o em [dry-run](#dry-run-mode) por mais tempo do que você executaria um template somente leitura, e coloque `edit_comment` atrás de [approval](#approval-workflow) até confiar no julgamento do modelo no seu tráfego.

### Triggers

- **Comment edited** (`COMMENT_EDIT`) - o agente compara o texto novo e o anterior e decide se a edição distorce respostas que já existem.

Veja [Trigger: Comment Edited](#trigger-comment-edit) para o payload completo, incluindo o texto do comentário anterior e a contagem de respostas no momento da edição.

### Allowed tools

- [`edit_comment`](#tool-edit-comment) - usado para restaurar o texto original quando a edição é julgada como gaslighting.
- [`warn_user`](#tool-warn-user) - emite um aviso leve que o usuário vê na próxima visita.
- [`send_dm`](#tools-overview) - o canal de explicação; o usuário recebe uma mensagem direta descrevendo por que a edição foi revertida.

Ele não pode banir, marcar como spam, votar ou postar novos comentários — a superfície é intencionalmente limitada.

### Recommended additions before going live

- **Coloque `edit_comment` atrás de [approval](#approval-workflow).** Reverter um comentário é visível para o autor e para qualquer pessoa que tenha visto a versão editada, então um falso positivo é constrangedor. Mantenha as aprovações ativadas até que o dry-run mostre que o agente é consistente.
- **Aperfeiçoe o prompt com o que conta como gaslighting no seu site.** O prompt padrão é curto de propósito. Dê ao modelo exemplos concretos — "inverter uma afirmação sim/não", "apagar um número citado por respostas", "adicionar uma frase hostil depois que respostas foram postadas" — e não-exemplos explícitos como correções de digitação, limpeza de formatação ou adição de fontes.
- **Use a contagem de respostas do contexto do trigger.** Edições em comentários com zero respostas não podem distorcer uma conversa; o prompt deve instruir o modelo a pular esses casos.
- **Marque "Include commenter's trust factor, account age, ban history, and recent comments"** em [Context Options](#context-options). O modelo é bem menos agressivo quando pode ver uma conta de longa data em boa-fé.
- **Considere uma pequena janela de tolerância para edições no prompt.** Muitas edições nos primeiros 30-60 segundos são correções de digitação; instrua o modelo a ignorar edições tão rápidas.

### Recommended dry-run window

Execute por pelo menos duas semanas de tráfego real em [dry-run](#dry-run-mode) antes de passar para Habilitado, e revise toda edição sinalizada durante esse período. Use [Test Runs (Replays)](#test-runs-replays) para reproduzir os últimos 30 dias de edições contra o agente antes de entrar em produção.