Par défaut, les utilisateurs peuvent supprimer leurs propres commentaires. De plus, la suppression de leur commentaire supprime automatiquement
tous les commentaires enfants et transitoires dans le fil. Ce comportement est également en temps réel.

Vous pouvez restreindre cela de la manière suivante :

- À la place, anonymisez le commentaire supprimé (définir name et text sur `[deleted]` ou sur une valeur personnalisée).
- Ne pas autoriser la suppression de commentaires s'il y a des réponses. Un message d'erreur personnalisable est affiché.
- Restreindre la suppression, lorsqu'un commentaire a des réponses, aux seuls administrateurs et modérateurs.

Vous pouvez configurer cela via la section `Comment Thread Deletion` dans l'interface de personnalisation du widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---