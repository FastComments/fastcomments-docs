It is possible to ban users using certain email providers using wildcards.

Moguće je blokirati korisnike koji koriste određene pružatelje e‑mailova koristeći zamjenske znakove.

For example, if you find that all comments from **@bademail.com** are spam, you can simply ban
that whole email provider by entering "*@bademail.com" in the email input field when adding a banned user.

Na primjer, ako otkrijete da su svi komentari s **@bademail.com** spam, možete jednostavno blokirati cijelog pružatelja e‑maila unosom "*@bademail.com" u polje za e‑mail prilikom dodavanja blokiranog korisnika.

Note the "*" before the @ in the email.

Primijetite "*" prije @ u e‑mailu.

### Subdomains

A domain ban also covers every subdomain of that domain. Banning `*@bademail.com` also bans
`someone@mail.bademail.com` and `someone@eu.mail.bademail.com`, so there is no need to add a separate ban for each subdomain.

Zabrana domenu također pokriva svaku poddomenu tog domena. Blokiranje `*@bademail.com` također blokira `someone@mail.bademail.com` i `someone@eu.mail.bademail.com`, pa nije potrebno dodavati zasebnu zabranu za svaku poddomenu.

If you only want to ban a specific subdomain, enter that subdomain instead, for example `*@mail.bademail.com`. That ban
does not affect `someone@bademail.com`.

Ako želite blokirati samo određenu poddomenu, unesite tu poddomenu, na primjer `*@mail.bademail.com`. Ta zabrana ne utječe na `someone@bademail.com`.

### Banning a Domain From a Comment

You do not have to type the pattern yourself. When you ban a user from a comment on the Moderate Comments page, the ban dialog
has a "Ban All @domain Users" checkbox that creates the same `*@domain` ban for the commenter's email domain.

Ne morate ručno upisivati uzorak. Kada blokirate korisnika iz komentara na stranici Moderiraj komentare, dijalog za blokiranje ima potvrdni okvir "Ban All @domain Users" koji stvara istu `*@domain` zabranu za domenu e‑maila komentatora.

### Supported Patterns

The only supported wildcard form is a single `*` in place of the whole name part, followed by `@` and a domain. Other forms
are rejected when you try to save them:

Jedini podržani oblik zamjenskog znaka je pojedinačni `*` umjesto cijelog dijela imena, nakon čega slijedi `@` i domena. Ostali oblici se odbacuju kada pokušate spremiti:

- `*@*.bademail.com` is not needed, because `*@bademail.com` already covers subdomains.

- `*@*.bademail.com` nije potreban, jer `*@bademail.com` već pokriva poddomene.

- `name*@bademail.com` and `*bademail.com` are not supported.

- `name*@bademail.com` i `*bademail.com` nisu podržani.

---