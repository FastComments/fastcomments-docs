[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments vise en notifikationsklokke øverst til højre i kommentarfeltet.

Denne klokke bliver rød og viser antallet af notifikationer, brugeren har. Eksempler på notifikationer er:

- En bruger svarede dig.
- En bruger svarede i en tråd, du har kommenteret i.
- En bruger stemte din kommentar op.
- En bruger svarede på en side, du har abonneret på.

Notifikationsklokken gør det også muligt at abonnere på en hel side.

Men vi kan deaktivere notifikationsklokken helt:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

Dette kan også gøres uden kode. På siden for tilpasning af widgeten, se afsnittet "Deaktiver notifikationsklokken".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]

---