Les commentaires peuvent être recherchés avec la syntaxe d'exemple suivante :

- Recherche floue par mots: `cats love`
- Correspondance de phrase exacte: `I love cats.`
- Correspondance exacte de l'intégralité du commentaire: `exact="I love cats."`
  - Correspond uniquement aux commentaires dont le texte entier correspond exactement à cette valeur (sensible à la casse), plutôt qu'aux commentaires qui la contiennent simplement.
- Par titre de page: `page:"Page Title"`
  - Prend en charge l'autocomplétion.
- Par URL de page: `page:"https://example.com/some-page"`
  - Prend en charge l'autocomplétion.
- Par site/domaine: `site:mysite.com` or `domain:othersite.com`
- Par utilisateur: `user:"Bob"`
  - Prend en charge l'autocomplétion.

Vous pouvez partager les résultats de recherche avec d'autres modérateurs ou administrateurs en partageant l'URL de la page depuis la page de modération. La valeur du champ de recherche
sera incluse dans l'URL de votre navigateur après avoir cliqué sur "Aller".