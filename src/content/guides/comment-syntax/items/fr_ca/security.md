Il y a plusieurs aspects de sécurité lorsqu'on permet aux gens d'ajouter du contenu à un site web puis de rendre ce contenu sur de nombreux types d'appareils différents.

### Prévention des abus de formatage

Les gens peuvent écrire du contenu qui est intentionnellement visuellement distrayant et diminue la valeur des discussions en abusant du formatage de texte.

FastComments fait plusieurs choses pour prévenir les abus en matière de formatage:

- Les grands sauts de ligne consécutifs répétés sont réduits.
- Nous ne rendons pas les titres (ils deviennent du texte normal).
- Nous n'autorisons pas le CSS ou les couleurs personnalisées.

### Prévention des exploits

Des exploits peuvent être créés dans les systèmes qui rendent du HTML. FastComments fait plusieurs choses pour prévenir cela:

- Nous n'autorisons qu'un ensemble explicitement défini de balises HTML.
- Nous n'autorisons qu'un ensemble explicitement défini d'attributs de balises HTML.
- Nous purifions et assainissons toutes les entrées.
  - Cela est fait via les bibliothèques [DOMPurify](https://www.npmjs.com/package/dompurify) et [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Nous avons choisi ces bibliothèques car elles sont bien testées (avec plus de 4 et 1 million de téléchargements par semaine, respectivement).

Cela signifie que les utilisateurs ne peuvent pas faire des choses comme écrire des balises `<script>` ou `<style>`, ou essayer d'ajouter des scripts de type `onload=alert()` aux images ou autre contenu.

Les balises HTML que nous autorisons sont les suivantes:

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
