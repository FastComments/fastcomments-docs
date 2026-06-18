---
Existem múltiplos aspectos de segurança ao permitir que pessoas adicionem conteúdo a um site
e então renderizar esse conteúdo em muitos tipos diferentes de dispositivos.

### Prevenção de Abuso de Formatação

Pessoas podem escrever conteúdo que é intencionalmente visualmente distrativo
e reduz o valor das discussões ao abusar da formatação de texto.

O FastComments faz várias coisas para prevenir abusos relacionados à formatação:

- Múltiplas quebras de linha consecutivas são colapsadas.
- Não renderizamos cabeçalhos (eles se tornam texto comum).
- Não permitimos CSS ou cores personalizadas.

### Prevenção de Exploits

Explorações podem ser criadas em sistemas que renderizam HTML. O FastComments faz várias coisas para prevenir isso:

- Só permitimos um conjunto de tags HTML explicitamente definido.
- Só permitimos um conjunto de atributos de tags HTML explicitamente definido.
- Purificamos e sanitizamos todas as entradas.
  - Isso é feito via as bibliotecas [DOMPurify](https://www.npmjs.com/package/dompurify) e [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Escolhemos essas bibliotecas por serem bem testadas (tendo mais de 4 e 1 milhão de downloads por semana, respectivamente).

Isto significa que os usuários não podem fazer coisas como escrever `<script>` ou `<style>` tags, ou tentar adicionar scripts do tipo `onload=alert()` em imagens ou outros conteúdos.

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

A tag `<iframe>` não é permitida por padrão. Se você ativar Permitir Incorporações de Mídia, iframes também são permitidos, mas somente quando sua fonte for um dos provedores confiáveis incorporados (como YouTube, Vimeo, SoundCloud e Spotify) ou um hostname que você tenha adicionado explicitamente. Iframes de qualquer outra fonte são removidos.
---