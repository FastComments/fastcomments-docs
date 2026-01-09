---
Podrazumevano, FastComments dozvoljava korisnicima da uređuju svoje komentare.

Međutim, moguće je to onemogućiti.

Na stranici za prilagođavanje widgeta, pogledajte opciju "Onemogući uređivanje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; title='Disable Comment Editing' app-screenshot-end]

- Ovo utiče samo na obične komentatore i ne utiče na moderatore ili admina, koji će i dalje moći da uređuju.
- Ovo će takođe uticati na API integracije kada se prosledi `contextUserId`.