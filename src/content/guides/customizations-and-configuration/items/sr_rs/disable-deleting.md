---
Podrazumevano, FastComments će dozvoliti korisnicima da obrišu svoje komentare.

Međutim, moguće je sprečiti to.

Na stranici za prilagođavanje widgeta, pogledajte opciju "Disable Deleting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; alt='Opcija onemogućavanja brisanja na stranici za prilagođavanje widgeta, sprečava komentatore da uklone svoje komentare'; title='Onemogući brisanje komentara' app-screenshot-end]

- Ovo utiče samo na obične komentatore, a ne na moderatore ili administratore, koji i dalje mogu da brišu.
- Ovo će takođe uticati na API integracije kada se prosleđuje `contextUserId`. 

---