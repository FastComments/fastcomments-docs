---
Par défaut, les utilisateurs peuvent supprimer leurs propres commentaires. De plus, la suppression de leur commentaire supprime automatiquement tous les commentaires enfants et transitoires du fil. Ce comportement est également en direct.

Vous pouvez restreindre cela de les manières suivantes :

- À la place, anonymisez le commentaire supprimé (définissez le nom et le texte sur `[deleted]` ou une valeur personnalisée).
- N'autorisez pas la suppression des commentaires lorsqu'il y a des réponses. Un message d'erreur personnalisable est affiché.
- Restreignez la suppression lorsqu'un commentaire a des réponses aux seuls administrateurs et modérateurs.

Cela peut être configuré via la section `Comment Thread Deletion` dans l'interface de personnalisation du widget.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-thread-deletion-mode']; selector = '.comment-thread-deletion-mode'; alt='Options de suppression de fil de commentaires dans l\'interface de personnalisation du widget pour anonymiser ou restreindre les suppressions avec réponses'; title='Personnaliser le comportement de suppression pour les réponses' app-screenshot-end]

---