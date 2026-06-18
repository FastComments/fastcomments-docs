Il existe plusieurs aspects de la sécurité lorsqu'on permet aux personnes d'ajouter du contenu à un site Web et ensuite d'afficher ce contenu sur différents types d'appareils.

### Prévention des abus de formatage

Les gens peuvent rédiger du contenu délibérément visuellement distrayant qui diminue la valeur des discussions en abusant du formatage du texte.

FastComments fait un certain nombre de choses pour prévenir les abus liés au formatage :

- Les longues séquences de sauts de ligne consécutifs sont compressées.
- Nous n'affichons pas les titres (ils deviennent du texte normal).
- Nous n'autorisons pas le CSS ni les couleurs personnalisées.

### Prévention des exploits

Des exploits peuvent être créés dans des systèmes qui rendent du HTML. FastComments fait plusieurs choses pour l'empêcher :

- Nous n'autorisons qu'un ensemble explicitement défini de balises HTML.
- Nous n'autorisons qu'un ensemble explicitement défini d'attributs de balises HTML.
- Nous purifions et assainissons toutes les entrées.
  - Cela est fait via les bibliothèques [DOMPurify](https://www.npmjs.com/package/dompurify) et [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Nous avons choisi ces bibliothèques car elles sont bien éprouvées (ayant respectivement plus de 4 millions et 1 million de téléchargements par semaine).

Cela signifie que les utilisateurs ne peuvent pas faire des choses comme écrire des balises `<script>` ou `<style>`, ni essayer d'ajouter des scripts de type `onload=alert()` aux images ou à d'autres contenus.

Les balises HTML que nous autorisons sont les suivantes :

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

Le tag `<iframe>` n'est pas autorisé par défaut. Si vous activez Allow Media Embeds, les iframes sont également permises, mais uniquement lorsque leur source fait partie d'une liste intégrée de fournisseurs de confiance (tels que YouTube, Vimeo, SoundCloud et Spotify) ou d'un nom d'hôte que vous avez explicitement ajouté. Les iframes provenant de toute autre source sont supprimées.