---
Podrazumevano, FastComments dozvoljava korisnicima da uređuju svoje komentare.

Međutim, moguće je sprečiti to.

Na stranici za prilagođavanje widgeta, pogledajte opciju "Disable Editing".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-edit'; selector = '.disable-commenter-comment-edit'; alt='Opcija onemogućavanja uređivanja na stranici prilagođavanja widgeta, sprečava komentatore da uređuju svoje komentare'; title='Onemogući uređivanje komentara' app-screenshot-end]

- Ovo utiče samo na obične komentatore i ne na moderatore ili administratore, koji i dalje mogu da uređuju.
- Ovo će takođe uticati na API integracije kada se prosleđuje `contextUserId`. 

---