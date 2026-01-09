---
Domyślnie FastComments pozwala użytkownikom edytować ich komentarze.

Jednak można temu zapobiec.

Na stronie dostosowywania widżetu zobacz opcję "Wyłącz edycję".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- To dotyczy tylko zwykłych komentujących, a nie moderatorów ani administratorów, którzy nadal będą mogli edytować.
- To będzie miało również wpływ na integracje API, gdy zostanie przekazane `contextUserId`. 

---