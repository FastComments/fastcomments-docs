---
Per impostazione predefinita, FastComments mostrerà il nome dell'utente così come è stato inserito, o come ci è stato trasmesso tramite SSO.

Tuttavia, potrebbe essere desiderabile mascherare o mostrare il nome dell'utente in modo diverso. Per esempio, se il nome dell'utente è Allen Rex, potresti voler mostrare solo "Allen R.".

Questo può essere fatto senza codice nell'interfaccia di personalizzazione del widget, sotto l'impostazione chiamata `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; alt='Menu a discesa Formato Nome Commentatore aperto con scelte come Capitalize, Last Initial e All Initials'; title='Modifica Formato Nome' app-screenshot-end]

I formati disponibili sono:

- Capitalize (mostra l'utente di esempio come Example User)
- Last Initial (mostra Example User come Example U.)
- All Initials (mostra Example User come E. U.)
- Mostra "Anonymous"

L'effetto di questa modifica è immediato. Gli utenti vedranno ancora il loro nome utente completo nella parte superiore dell'area dei commenti, per loro stessi, ma i loro commenti mostreranno il nome utente modificato.

I nomi utente sono mascherati lato server per proteggere gli utenti.

---