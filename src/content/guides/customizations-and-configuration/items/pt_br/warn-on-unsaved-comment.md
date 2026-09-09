[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Por padrão, se um usuário digitar um comentário e então atualizar a página, fechar a aba ou navegar para outro lugar antes de enviá‑lo, o rascunho é perdido silenciosamente.

Definir **warnOnUnsavedComment** como true faz com que o navegador peça ao usuário que confirme antes de sair da página enquanto qualquer caixa de comentário, ou uma edição em andamento, ainda contiver texto. Uma vez que o comentário é enviado, o texto é limpo, portanto nenhum prompt é exibido.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Avisar sobre comentário não salvo'; code-example-end]

O prompt usa a própria caixa de diálogo do navegador. Navegadores modernos exibem sua própria redação e ignoram texto personalizado, portanto a mensagem não pode ser customizada.

Esta opção carrega uma pequena extensão sob demanda, portanto não adiciona nada ao widget para sites que não a habilitam.