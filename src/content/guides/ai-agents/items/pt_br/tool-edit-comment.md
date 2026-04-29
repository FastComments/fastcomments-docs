A ferramenta Edit permite que o agente substitua o texto de um comentário existente. É destrutiva de uma forma que a maioria das outras ferramentas de moderação não é: ela sobrescreve conteúdo escrito pelo usuário. Reserve-a para casos restritos e bem definidos.

### O que faz

O agente envia um ID de comentário e um corpo de substituição. A plataforma grava o novo texto no comentário e registra uma entrada `TextChanged` no log de auditoria do comentário, capturando tanto o texto antigo quanto o novo. O original nunca é perdido - os moderadores podem ler o que o comentário dizia antes de o agente editá-lo.

### Escopo

Como toda ferramenta que modifica comentários, o Edit está restrito à allowlist do gatilho - o agente só pode editar o comentário no qual o gatilho foi acionado, seu pai, ou outro comentário dentro do escopo a partir do mesmo contexto de gatilho. Uma tentativa de prompt-injection para "edit comment XYZ" onde XYZ é não relacionado será recusada no servidor antes do executor ser executado.

### Laços

Quando o agente edita um comentário, a plataforma dispara um gatilho `COMMENT_EDIT` como faria para uma edição humana, mas **suprime o envio para outros agentes**. Isso evita que dois agentes que ambos escutam `COMMENT_EDIT` entrem em ping-pong com as edições um do outro.

### Quando permitir

Para agentes que lidam com remoção de PII, ou para agentes de sumarização/digest que se autoeditam. A maioria dos agentes de moderação **não** precisa desta ferramenta - mark-spam, warn, e ban cobrem o ciclo de vida típico.

### Aprovações

**Considere fortemente colocar por trás de aprovação**, especialmente enquanto você está construindo confiança no agente. Um agente reescrevendo as palavras de um usuário é o tipo de ação que a comunidade notará e reagirá, e é mais difícil "desfazer" reputacionalmente do que uma exclusão.

### Veja também

- [Trigger: Comment Edited](#trigger-comment-edit) - o gatilho disparado quando o texto de um comentário muda.
- [Approval Workflow](#approval-workflow) - como colocar a ferramenta sob revisão humana.