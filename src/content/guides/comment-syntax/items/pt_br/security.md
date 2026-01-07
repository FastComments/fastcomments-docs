Existem múltiplos aspectos de segurança quando permitimos que pessoas adicionem conteúdo a um site e depois renderizem esse conteúdo em muitos tipos diferentes de dispositivos.

### Prevenção de abuso de formatação

As pessoas podem escrever conteúdo que é intencionalmente distrativo visualmente e diminui o valor das discussões ao abusar da formatação de texto.

FastComments faz várias coisas para prevenir abuso em relação à formatação:

- Grandes quebras de linha consecutivas repetidas são colapsadas.
- Não renderizamos títulos (eles se tornam texto normal).
- Não permitimos CSS ou cores personalizadas.

### Prevenção de exploits

Exploits podem ser criados em sistemas que renderizam HTML. FastComments faz várias coisas para prevenir isso:

- Permitimos apenas um conjunto explicitamente definido de tags HTML.
- Permitimos apenas um conjunto explicitamente definido de atributos de tags HTML.
- Purificamos e sanitizamos todas as entradas.
  - Isso é feito através das bibliotecas [DOMPurify](https://www.npmjs.com/package/dompurify) e [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Escolhemos essas bibliotecas por serem bem testadas (com mais de 4 e 1 milhão de downloads por semana, respectivamente).

Isso significa que os usuários não podem fazer coisas como escrever tags `<script>` ou `<style>`, ou tentar adicionar scripts do tipo `onload=alert()` a imagens ou outro conteúdo.

As tags HTML que permitimos são as seguintes:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`
