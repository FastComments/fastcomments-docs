Per impostazione predefinita, FastComments mostrerà il nome dell'utente così come è stato inserito, o come ci è stato passato tramite SSO.

Tuttavia, può essere desiderabile mascherare o visualizzare il nome dell'utente in modo diverso. Ad esempio, se il nome dell'utente è Allen Rex, potresti voler mostrare solo "Allen R.".

Questo può essere fatto senza codice nell'interfaccia di personalizzazione del widget, nell'impostazione chiamata `Commenter Name Format`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.commenter-name-format select'; selector = '.commenter-name-format'; title='Change Name Format' app-screenshot-end]

I formati disponibili sono:

- Capitalize (visualizza Example User come Example User)
- Last Initial (visualizza Example User come Example U.)
- All Initials (visualizza Example User come E. U.)
- Show "Anonymous"

L'effetto della modifica è immediato. Gli utenti vedranno ancora il loro nome utente completo in cima all'area dei commenti, per se stessi, ma i loro commenti mostreranno il nome utente modificato.

I nomi utente vengono mascherati lato server per proteggere gli utenti.