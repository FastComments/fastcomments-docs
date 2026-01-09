---
Po podrazumevanoj postavci, FastComments dozvoljava korisnicima da brišu svoje komentare.

Međutim, moguće je to onemogućiti.

Na stranici za prilagođavanje widgeta, pogledajte opciju "Onemogući brisanje".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.disable-commenter-comment-delete'; selector = '.disable-commenter-comment-delete'; title='Disable Comment Deleting' app-screenshot-end]

- Ovo utiče samo na obične komentatore i ne odnosi se na moderatore ili administratore, koji će i dalje moći da brišu.
- Ovo će takođe uticati na API integracije kada se prosledi `contextUserId`. 

---