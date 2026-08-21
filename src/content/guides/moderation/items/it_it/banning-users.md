There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">utenti bannati</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Elenco degli utenti bannati sotto Moderazione Commenti, con gli indirizzi email bannati e un pulsante per aggiungere un nuovo ban'; title='La pagina degli utenti bannati' app-screenshot-end]

This page can be accessed via Moderate Comments -> Banned Users

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Nuovo modulo di ban con un campo email e una scelta del tipo di ban: Permanente o Ban Ombra Permanente'; title='Bannare un Utente' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### Alias Email

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will
also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### Ban Ombra

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be desirable in certain situations.

### Bannare tramite Indirizzo IP

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

### Ricerca Utenti Bannati

Once your list grows past a page or two, you can narrow it with the search row above the table.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Riga di ricerca nella pagina Utenti Bannati con un menu a tendina Cerca per, un menu a tendina Corrispondenza, e un campo di input Valore'; title='Ricerca Utenti Bannati' app-screenshot-end]

There are three controls:

- **Cerca per** picks which field to look in: Qualsiasi Campo, Email, Nome, Bannato da, or Bannato per Dire. The last four correspond to the columns of the same name in the table.
- **Corrispondenza** picks how to compare. **Contiene** finds your value anywhere in the field, and **Uguale** matches the whole field.
- **Valore** is the text to look for.

Every field is matched without regard to case, so searching for `SPAMMER@EXAMPLE.COM` finds a ban stored as `spammer@example.com`.

A few things worth knowing:

- **Bannato per Dire** searches the text of the comment that got the user banned. This is how you find everyone banned over a particular phrase.
- **Bannato da** searches the name of the moderator who issued the ban, which is useful for reviewing another moderator's decisions.
- Wildcard bans are stored with their `*`, so a **Contiene** search for `bademail.com` finds a `*@bademail.com` ban.
- **Nome** matches the name shown in the Name column, so it finds a user even if they have changed their name since being banned, and even if you created the ban by entering an email address and no name was recorded at the time. The name recorded on the ban still matches too, so searching for either the old or the current name works.
- **Qualsiasi Campo** searches the email, name, banned-by moderator, and banned comment text together.

Your search is part of the page URL, so you can share a filtered list with other moderators the same way you share other moderation links. Paging through results keeps the search applied, starting a new search returns you to the first page, and **Cancella** returns to the full list.