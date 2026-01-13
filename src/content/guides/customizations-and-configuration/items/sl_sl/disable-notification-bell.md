[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments prikazal zvonec za obvestila v zgornjem desnem kotu območja komentarjev.

Ta zvonec bo postal rdeč in prikazal število obvestil, ki jih ima uporabnik. Nekateri primeri obvestil so:

- Uporabnik vam je odgovoril.
- Uporabnik je odgovoril v niti, v kateri ste komentirali.
- Uporabnik je dal vašemu komentarju glas.
- Uporabnik je odgovoril na stran, na katero ste naročeni.

Zvonec za obvestila omogoča tudi mehanizem za naročanje na celotno stran.

Vendar lahko povsem onemogočimo zvonec za obvestila:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Disable Notification Bell'; code-example-end]

To je mogoče narediti tudi brez kode. Na strani za prilagajanje vtičnika si oglejte razdelek "Onemogoči zvonec za obvestila".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; title='Disable Notification Bell' app-screenshot-end]