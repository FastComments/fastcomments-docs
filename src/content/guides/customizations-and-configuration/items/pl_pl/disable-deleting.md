---
Domyślnie FastComments pozwala użytkownikom usuwać ich komentarze.

Jednak możliwe jest zapobieżenie temu.

Na stronie dostosowywania widgetu zobacz opcję "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Opcja Disable Deleting na stronie dostosowywania widgetu, zapobiegająca usuwaniu komentarzy przez komentujących'; title='Wyłączanie usuwania komentarzy' app-screenshot-end]

- To wpływa tylko na zwykłych komentujących, a nie na moderatorów ani administratorów, którzy nadal będą mogli usuwać.
- To również wpłynie na integracje API, gdy przekazywany jest `contextUserId`. 

---