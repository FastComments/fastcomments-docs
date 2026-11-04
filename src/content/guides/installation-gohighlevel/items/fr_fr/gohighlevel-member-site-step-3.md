Nous allons maintenant générer votre code FastComments personnalisé. Utilisez l'assistant ci-dessous pour configurer le fonctionnement de FastComments sur votre site GoHighLevel :

[snippet id="gohighlevel-wizard"]

### Différents types de boîtes de commentaires

Vous pouvez configurer la ligne `TYPE = 'commenting'` pour changer le produit utilisé (par exemple, vous pouvez la remplacer par `live` pour un chat de streaming ou `collab` pour un chat collaboratif).

### Placer la boîte de commentaires où vous le souhaitez

Supposons que vous souhaitiez placer des boîtes de commentaires sur des parties spécifiques de la page et non aux emplacements par défaut.
Modifiez cette ligne :

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

En :

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Ensuite, dans l'éditeur GHL, cliquez sur le bouton « code » et ajoutez l'endroit où vous souhaitez que les commentaires apparaissent :

[inline-code-attrs-start title = 'Div FastComments pour GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Type de boîte de commentaires différent par page

Supposons que vous souhaitiez que les utilisateurs surlignent et discutent des morceaux de texte, ou utilisent l'interface de chat de streaming à la place.

Suivez d'abord les étapes ci-dessus dans « Placer la boîte de commentaires où vous le souhaitez ».

Notez que dans ce petit extrait, il y a `type="commenting"`.

Si vous souhaitez activer le chat collaboratif par exemple, remplacez le type par `type="collab"`.

### Afficher uniquement sur des pages spécifiques

Si vous ne définissez pas `TARGET_ELEMENT_ID`, vous pouvez plutôt configurer la variable `VALID_PATTERNS`, pour définir les routes d'URL sur lesquelles les commentaires doivent s'afficher. Par défaut, ils s'afficheront
sur les pages dont l'URL contient `/post`.

### Configuration du chat collaboratif

Vous pouvez indiquer au chat collaboratif de n'ajouter les fonctionnalités collaboratives qu'autour du HTML d'une zone spécifique. Par exemple, supposons que vous
ajoutiez le code de pied de page ci-dessus, puis que vous ajoutiez cette div dans le contenu de l'article/de la page pour activer le chat collaboratif :

[inline-code-attrs-start title = 'Chat collaboratif avec contenu spécifié'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Alors l'élément paragraphe à l'intérieur de la `<div>` aura le chat collaboratif activé, et rien d'autre sur la page. Si vous ne
mettez aucun contenu dans la `<div>`, le chat collaboratif sera activé sur l'ensemble du corps de l'article.