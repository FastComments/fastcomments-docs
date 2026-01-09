### Como a Seleção de Texto Funciona

Quando os usuários selecionam texto dentro do contêiner do Collab Chat, o widget captura essa seleção e permite que eles iniciem uma discussão. A seleção pode ser tão pequena quanto uma única palavra ou tão grande quanto vários parágrafos que abrangem diferentes elementos.

### Atraso na Seleção

Em desktops, há um atraso de 3,5 segundos entre o momento em que um usuário seleciona o texto e quando o prompt de discussão aparece. Isso evita que a interface pisque quando os usuários estão apenas destacando texto para copiar ou ler. Em dispositivos móveis, o prompt aparece imediatamente, já que a seleção de texto é mais intencional em telas sensíveis ao toque.


### IDs exclusivos de conversa

Cada conversa recebe um `urlId` único que combina a URL da página, o caminho do elemento no DOM e o intervalo de texto serializado. Isso garante que cada seleção de texto crie uma conversa distinta que possa ser encontrada novamente depois.

Se você fornecer um `urlId` personalizado na sua configuração, ele será combinado com o caminho do elemento e o intervalo de texto para criar o identificador final.

### Destaques Visuais

Quando existe uma discussão para uma seleção de texto específica, esse texto recebe um destaque visual. O destaque é implementado usando cores de fundo e aparece ao passar o cursor ou quando a janela de chat associada está aberta.

O sistema de destaque funciona envolvendo o texto selecionado em um elemento especial que pode ser estilizado. Essa abordagem garante que os destaques permaneçam precisos mesmo quando a estrutura HTML subjacente é complexa.

### Posicionamento da Janela de Chat

Quando um usuário clica em um destaque ou cria uma nova anotação, uma janela de chat aparece perto do texto selecionado. O widget calcula automaticamente a melhor posição para essa janela com base no espaço disponível na viewport.

O sistema de posicionamento usa classes CSS como `to-right`, `to-left`, `to-top` e `to-bottom` para indicar em que direção a janela de chat deve se estender a partir do destaque. Em dispositivos móveis (telas com menos de 768px de largura), a janela de chat sempre aparece em tela cheia para melhor usabilidade.

### Dimensões da Janela de Chat

As janelas de chat têm 410px de largura em desktops, com espaçamento de 20px e uma seta visual de 16px apontando para o texto destacado. Essas dimensões são fixas para garantir comportamento consistente, mas você pode personalizar a aparência com CSS.

### Seleções entre Elementos

Os usuários podem selecionar texto que abrange múltiplos elementos HTML, como destacar do meio de um parágrafo até o início de outro. O sistema de serialização de intervalo lida com isso corretamente e destacará todo o texto selecionado mesmo através das fronteiras dos elementos.

### Compatibilidade com Navegadores

O sistema de seleção de texto usa a API padrão `window.getSelection()`, que é suportada em todos os navegadores modernos. Para versões antigas do Internet Explorer, ele recorre a `document.selection` para compatibilidade.

### Persistência da Seleção

Uma vez que uma conversa é criada para uma seleção de texto, essa anotação persiste mesmo se a página for recarregada. O intervalo serializado e o caminho no DOM permitem que o widget restaure os destaques exatamente na mesma localização quando os usuários retornarem à página.

Isso funciona de forma confiável desde que o conteúdo da sua página permaneça estável. Se você alterar o conteúdo de texto ou reestruturar seu HTML, as anotações existentes podem não mais se alinhar corretamente com o texto. Por esse motivo, é melhor evitar mudanças significativas no conteúdo em páginas com anotações ativas, ou considerar migrar as anotações quando alterações no conteúdo forem necessárias.