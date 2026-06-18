Il y a plusieurs aspects de la sécurité lorsqu'on permet aux utilisateurs d'ajouter du contenu sur un site web
et ensuite d'afficher ce contenu sur de nombreux types d'appareils.

### Empêcher les abus de mise en forme

Les personnes peuvent rédiger du contenu intentionnellement visuellement distrayant
et réduire la valeur des discussions en abusant de la mise en forme du texte.

FastComments met en place plusieurs mesures pour prévenir les abus liés à la mise en forme :

- Les grandes séquences de sauts de ligne consécutifs sont compressées.
- Nous n'affichons pas les titres (ils deviennent du texte normal).
- Nous n'autorisons pas le CSS ni les couleurs personnalisées.

### Prévention des exploits

Des exploits peuvent être créés dans des systèmes qui rendent du HTML. FastComments prend plusieurs mesures pour les empêcher :

- Nous n'autorisons qu'un ensemble explicitement défini de balises HTML.
- Nous n'autorisons qu'un ensemble explicitement défini d'attributs de balises HTML.
- Nous purifions et assainissons toutes les entrées.
  - Cela est réalisé via les bibliothèques [DOMPurify](https://www.npmjs.com/package/dompurify) et [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Nous avons choisi ces bibliothèques car elles sont bien testées (ayant respectivement plus de 4 et 1 million de téléchargements par semaine).

Cela signifie que les utilisateurs ne peuvent pas faire des choses comme écrire des balises `<script>` ou `<style>`, ou tenter d'ajouter des scripts du type `onload=alert()` aux images ou à d'autres contenus.

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

La balise `<iframe>` n'est pas autorisée par défaut. Si vous activez Autoriser l'intégration de médias, les iframes sont également autorisés, mais uniquement lorsque leur source figure parmi une liste intégrée de fournisseurs de confiance (tels que YouTube, Vimeo, SoundCloud et Spotify) ou un nom d'hôte que vous avez explicitement ajouté. Les iframes provenant de toute autre source sont supprimés.

---