There are two ways to ban users from commenting on your site with FastComments.

The first is if you already know their email, you can enter it on the <a href="https://fastcomments.com/auth/my-account/moderate-comments/banned-users" target="_blank">zabranjeni korisnici</a> page.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .account-block'; alt='Lista zabranjenih korisnika pod Moderirani komentari, sa zabranjenim email adresama i dugmetom za dodavanje nove zabrane'; title='Stranica zabranjenih korisnika' app-screenshot-end]

This page can be accessed via Moderirani komentari -> Zabranjeni korisnici

When we go to ban a user, we can pick a type, either Permanent or Permanent Shadow Ban:

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users/new'; selector = '.content .account-block'; alt='Novi formular za zabranu sa poljem za email i izborom tipa zabrane: Trajna ili Trajna senka zabrana'; title='Zabranjivanje korisnika' app-screenshot-end]

The second way to ban a user is by clicking the ban button that is placed on each comment on the Comment Moderation page.

When we click the ban button, you'll be presented with some options, where we can specify the ban type and duration.

### Email aliasi

When banning a user by email, FastComments automatically ignores `+` aliases. For example, banning `user+alias@gmail.com` will also ban `user@gmail.com` and any other `+` variation of that address, such as `user+other@gmail.com`.

### Senka zabrane

A shadow-ban is a type of ban that makes it appear that the user's comment or vote was saved successfully, when in fact it was not. This may be desirable in certain situations.

### Zabrana putem IP adrese

Unless a tenant wishes to opt out, FastComments supports banning via IP by storing a hashed version of the commenter's IP address.

### Pretraga zabranjenih korisnika

Once your list grows past a page or two, you can narrow it with the search row above the table.

[app-screenshot-start url='/auth/my-account/moderate-comments/banned-users'; selector = '.content .filter-form'; alt='Red za pretragu na stranici Zabranjenih korisnika sa padajućim menijem \'Search By\', padajućim menijem \'Match\' i poljem \'Value\''; title='Pretraga zabranjenih korisnika' app-screenshot-end]

There are three controls:

- **Search By** picks which field to look in: Any Field, Email, Name, Banned By, or Banned For Saying. The last four correspond to the columns of the same name in the table.
- **Match** picks how to compare. **Contains** finds your value anywhere in the field, and **Equals** matches the whole field.
- **Value** is the text to look for.

Every field is matched without regard to case, so searching for `SPAMMER@EXAMPLE.COM` finds a ban stored as `spammer@example.com`.

A few things worth knowing:

- **Banned For Saying** searches the text of the comment that got the user banned. This is how you find everyone banned over a particular phrase.
- **Banned By** searches the name of the moderator who issued the ban, which is useful for reviewing another moderator's decisions.
- Wildcard bans are stored with their `*`, so a **Contains** search for `bademail.com` finds a `*@bademail.com` ban.
- **Name** matches the name shown in the Name column, so it finds a user even if they have changed their name since being banned, and even if you created the ban by entering an email address and no name was recorded at the time. The name recorded on the ban still matches too, so searching for either the old or the current name works.
- **Any Field** searches the email, name, banned-by moderator, and banned comment text together.

Your search is part of the page URL, so you can share a filtered list with other moderators the same way you share other moderation links. Paging through results keeps the search applied, starting a new search returns you to the first page, and **Clear** returns to the full list.