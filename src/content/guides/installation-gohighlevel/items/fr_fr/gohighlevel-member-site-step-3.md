Maintenant nous allons générer votre code FastComments personnalisé. Utilisez l'assistant ci-dessous pour configurer la façon dont vous voulez que FastComments fonctionne sur votre site GoHighLevel :

[snippet id="gohighlevel-wizard"]

### Different Comment Box Types

You can configure the `TYPE = 'commenting'` line to switch the product used (for example you can change it to `live` for streaming chat or `collab` for collab chat).

### Putting The Comment Box Where You Want

Supposons que vous souhaitiez placer des boîtes de commentaires sur des parties spécifiques de la page et non aux emplacements par défaut.
Modifiez cette ligne :

    const TARGET_ELEMENT_ID = ''; // définir pour utiliser le mode div ciblé

Remplacez par :

    const TARGET_ELEMENT_ID = 'fc_box'; // définir pour utiliser le mode div ciblé

Ensuite, dans l'éditeur GHL, cliquez le bouton "code" et ajoutez l'endroit où vous voulez que les commentaires apparaissent :

[inline-code-attrs-start title = 'Div FastComments pour GoHighLevel'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="commenting"
  urlid="custom-chat-id"
></div>
[inline-code-end]

### Different Comment Box Type Per-Page

Supposons que vous souhaitiez que les utilisateurs surlignent et discutent des passages de texte, ou utilisent plutôt l'interface de chat en streaming.

Suivez d'abord les étapes ci-dessus dans "Placer la boîte de commentaires où vous le souhaitez".

Notez que dans ce petit extrait il y a `type="commenting"`.

Si vous souhaitez activer le chat collaboratif, par exemple, changez le type en `type="collab"`.

### Only Show On Specific Pages

Si vous ne définissez pas `TARGET_ELEMENT_ID`, vous pouvez configurer à la place la variable `VALID_PATTERNS` pour définir sur quelles routes URL les commentaires doivent s'afficher. Par défaut, ils s'afficheront sur les pages dont l'URL contient `/post`.

### Configuring Collab Chat

Vous pouvez indiquer au chat collaboratif d'ajouter la fonctionnalité collaborative uniquement autour du HTML à l'intérieur d'une zone spécifique. Par exemple, supposons que vous ajoutiez le code de pied de page ci-dessus, puis ajoutiez ce div dans le contenu de l'article/de la page pour activer le chat collaboratif :

[inline-code-attrs-start title = 'Chat collaboratif avec contenu spécifié'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<div
  id="fc_box"
  type="collab"
  urlid="custom-chat-id"
><p>This content will have collab chat!</p></div>
[inline-code-end]

Ensuite, l'élément paragraphe à l'intérieur du `<div>` aura le chat collaboratif activé, et rien d'autre sur la page. Si vous ne mettez aucun contenu dans le `<div>` alors cela activera le chat collaboratif sur l'intégralité du corps de l'article.

---