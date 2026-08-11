---
Domyślnie FastComments pozwala użytkownikom edytować ich komentarze.

Jednak można to uniemożliwić.

Na stronie dostosowywania widgetu zobacz opcję „Wyłącz edycję”.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Opcja wyłączenia edycji na stronie dostosowywania widgetu, uniemożliwiająca komentatorom edytowanie ich komentarzy'; title='Wyłącz edycję komentarzy' app-screenshot-end]

- To dotyczy tylko zwykłych komentatorów, a nie moderatorów ani administratorów, którzy nadal będą mogli edytować.
- Będzie to również wpływać na integracje API, gdy przekazywany jest `contextUserId`. 

---