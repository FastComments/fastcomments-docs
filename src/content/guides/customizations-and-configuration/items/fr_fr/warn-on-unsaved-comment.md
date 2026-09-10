[related-parameter-start name = 'warnOnUnsavedComment'; type = 'boolean'; related-parameter-end]

Par défaut, si un utilisateur saisit un commentaire puis rafraîchit la page, ferme l’onglet ou quitte le site avant de le soumettre, le brouillon est perdu silencieusement.

Activer **warnOnUnsavedComment** à true fait que le navigateur demande à l’utilisateur de confirmer avant de quitter la page tant qu’une zone de commentaire ou une modification en cours contient du texte. Une fois le commentaire soumis, le texte est effacé, donc aucune invite n’est affichée.

[code-example-start config = {warnOnUnsavedComment: true}; linesToHighlight = [6]; title = 'Avertir sur le commentaire non enregistré'; code-example-end]

L’invite utilise la boîte de dialogue native du navigateur. Les navigateurs modernes affichent leur propre libellé et ignorent le texte personnalisé, de sorte que le message ne peut pas être modifié.

Cette option charge une petite extension à la demande, elle n’ajoute donc rien au widget pour les sites qui ne l’activent pas.