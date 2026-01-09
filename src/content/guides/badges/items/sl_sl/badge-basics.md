FastComments uporabniške značke konfigurirajo skrbniki s dovoljenjem `Customize Data`.

This is done via [Customize -> Badges](https://fastcomments.com/auth/my-account/configure-badges) in your admin dashboard.

Ko uporabniku dodelimo značko, se ta prikaže na njegovem profilu in ob njegovih komentarjih.

When adding a badge we can setup a `Display Label`, which is the name the user sees associated with the badge. For example, if we add a `Comment Count` badge
we probably don't want to show that technical name because it's very lame. We might call it `Super Member` or similar. Badges can also stack and replace each other, as we'll cover
later in this document.

Značke imajo tudi nastavljive pragove.

Badges can be created, and then later disabled by unchecking `Enabled`. Disabling a badge means it will no longer be automatically awarded, and won't show in the Award Manual Badge menu, but
users will keep the badge.

### Badge Display Types

Značke so lahko slikovne ali besedilne, ki podpirajo osnovno oblikovanje (barva besedila, barva ozadja in barva obrobe). Značke lahko tudi stilizirate preko CSS-a.

Image badges can be GIF images to show animated badges.

### Tip - Do Not Remove Badges!

Uporabniki obožujejo značke. Res jim veliko pomenijo, tudi če gre za napako, ki ste jo dodali po pomoti, in želite spremeniti ikono značke.

If we've learned anything, it's extremely difficult to take something away from users. Removing a badge because you as an owner of the site no
longer like it, or want to make changes, may result in a very angry crowd of users that suddenly leave your site out of frustration. For this reason
`Delete` was not even an option for the first few months we released this feature - however we ended up having to add it. But please, use delete with caution. We have
seen many long time, multi-year, users get very frustrated and leave their communities because administrators decided to delete a badge.

Če morate prenehati uporabljati značko, predlagamo, da jo preprosto onemogočite, da uporabniki obdržijo svojo značko.

### Badge Reprocessing

When a badge is added or changed, the system will retroactively check anyone that interacted with your site to see if they should get the badge. This will be
visible in the Badges page in the admin dashboard, as a spinner will be shown instead of the number of users that have the badge. This is because the number of users
is being determined.

### Seeing Who Has a Badge

In the Badges list each link has a `View Users` option to show the list of users which has earned or was manually awarded a badge.