Nous allons maintenant générer votre code FastComments personnalisé. Utilisez l'assistant ci‑dessous pour configurer la façon dont vous voulez que FastComments fonctionne sur votre site GoHighLevel :

[snippet id="gohighlevel-wizard"]

### Différents types de boîtes de commentaires

Vous pouvez configurer la ligne `TYPE = 'commenting'` pour changer le produit utilisé (par exemple, vous pouvez la modifier en `live` pour le chat en streaming ou `collab` pour le chat collaboratif).

### Placer la boîte de commentaires où vous le souhaitez

Supposons que vous souhaitiez placer des boîtes de commentaires sur des parties spécifiques de la page et non aux emplacements par défaut.
Changez cette ligne :

    const TARGET_ELEMENT_ID = ''; // set to use target div mode

Par :

    const TARGET_ELEMENT_ID = 'fc_box'; // set to use target div mode

Ensuite, dans l'éditeur GHL, cliquez sur le bouton "code" et ajoutez l'endroit où vous voulez que les commentaires apparaissent :

[inline-code-attrs-start title = 'Div FastComments pour GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Type de boîte de commentaires différent par page

Supposons que vous souhaitiez que les utilisateurs puissent surligner et discuter des passages de texte, ou utiliser plutôt l'interface de chat en streaming.

Commencez par suivre les étapes ci‑dessus dans "Placer la boîte de commentaires où vous le souhaitez".

Remarquez que dans ce petit extrait il y a `type="commenting"`.

Par exemple, si vous voulez activer le chat collab, changez le type en `type="collab"`.

### Afficher seulement sur des pages spécifiques

Si vous ne définissez pas `TARGET_ELEMENT_ID`, vous pouvez plutôt configurer la variable `VALID_PATTERNS` pour définir sur quelles routes d'URL les commentaires doivent s'afficher. Par défaut, ils s'affichent sur les pages contenant `/post` dans l'URL.

### Configurer le chat collab

Vous pouvez indiquer au chat collab de n'ajouter des fonctionnalités collaboratives qu'autour du HTML à l'intérieur d'une zone spécifique. Par exemple, supposons que vous ajoutiez le code du pied de page ci‑dessus, puis ajoutiez cette div dans le contenu de l'article/page pour activer le chat collab :

[inline-code-attrs-start title = 'Chat collab avec contenu spécifié'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Ensuite, l'élément de paragraphe à l'intérieur du `<div>` aura le chat collab activé, et rien d'autre sur la page. Si vous ne mettez aucun contenu dans le `<div>`, il activera le chat collab sur tout le corps de l'article.

---