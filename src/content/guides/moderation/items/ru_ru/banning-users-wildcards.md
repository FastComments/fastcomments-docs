It is possible to ban users using certain email providers using wildcards.

For example, if you find that all comments from **@bademail.com** are spam, you can simply ban
that whole email provider by entering "*@bademail.com" in the email input field when adding a banned user.

Note the "*" before the @ in the email.

### Поддомены

A domain ban also covers every subdomain of that domain. Banning `*@bademail.com` also bans
`someone@mail.bademail.com` and `someone@eu.mail.bademail.com`, so there is no need to add a separate ban for each subdomain.

If you only want to ban a specific subdomain, enter that subdomain instead, for example `*@mail.bademail.com`. That ban
does not affect `someone@bademail.com`.

### Запрет домена из комментария

You do not have to type the pattern yourself. When you ban a user from a comment on the Moderate Comments page, the ban dialog
has a "Ban All @domain Users" checkbox that creates the same `*@domain` ban for the commenter's email domain.

### Поддерживаемые шаблоны

The only supported wildcard form is a single `*` in place of the whole name part, followed by `@` and a domain. Other forms
are rejected when you try to save them:

- `*@*.bademail.com` is not needed, because `*@bademail.com` already covers subdomains.
- `name*@bademail.com` and `*bademail.com` are not supported.

---