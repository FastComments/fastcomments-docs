[related-parameter-start name = 'countAll'; type = 'boolean'; related-parameter-end]

A contagem de comentários exibida no topo do widget de comentários pode mostrar ou todos os comentários "top-level", ou seja, aquelas respostas que
são respostas diretamente para a página ou artigo em si, ou pode ser uma contagem de **todos** os comentários aninhados.

Por padrão, isso é `true` - é uma contagem do último caso - todos os comentários. Em versões antigas do widget de comentários o
valor padrão é `false`.

Podemos alterar o comportamento, de modo que seja uma contagem de **todos** os comentários aninhados definindo a flag **countAll** para `true`.

[code-example-start config = {countAll: true}; linesToHighlight = [6]; title = 'Counting All Comments'; code-example-end]

Se quiséssemos que a contagem refletisse apenas os comentários de nível superior, definimos a flag para `false`.

[code-example-start config = {countAll: false}; linesToHighlight = [6]; title = 'Counting Top Level Comments'; code-example-end]

Atualmente, isso não pode ser personalizado sem alterações no código.

---