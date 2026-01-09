Par défaut, les utilisateurs peuvent supprimer leurs propres commentaires. De plus, la suppression de leur commentaire supprime automatiquement tous les commentaires enfants et transitoires du fil. Ce comportement est également actif.

Vous pouvez restreindre cela de la manière suivante :

- À la place, anonymiser le commentaire supprimé (définir name et text sur `[deleted]` ou une valeur personnalisée).
- Ne pas autoriser la suppression de commentaires lorsqu'il y a des réponses. Un message d'erreur personnalisable est affiché.
- Restreindre la suppression lorsqu'un commentaire a des réponses uniquement aux administrateurs et modérateurs.

Cela peut être configuré via la section `Comment Thread Deletion` dans l'UI de personnalisation du widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; title='Customize Delete Behavior for Replies' app-screenshot-end]

---