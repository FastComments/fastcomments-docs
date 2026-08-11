[related-parameter-start name = 'disableNotificationBell'; type = 'boolean'; related-parameter-end]

Privzeto bo FastComments prikazal obvestilni zvonec v zgornjem desnem kotu območja komentarjev.

Zvonec bo postal rdeč in prikazal število obvestil, ki jih ima uporabnik. Nekateri primeri obvestil so:

- Uporabnik je odgovoril vam.
- Uporabnik je odgovoril v nitki, v kateri ste komentirali.
- Uporabnik je glasoval za vaš komentar.
- Uporabnik je odgovoril na stran, na katero ste se naročili.

Obvestilni zvonec prav tako omogoča naročanje na celotno stran.

Vendar pa lahko obvestilni zvonec popolnoma onemogočimo:

[code-example-start config = {disableNotificationBell: true}; linesToHighlight = [6]; title = 'Onemogoči obvestilno zvonec'; code-example-end]

To je mogoče tudi brez kode. Na strani za prilagajanje gradnika si oglejte razdelek "Onemogoči obvestilno zvonec".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-notification-bell']; selector = '.disable-notification-bell'; alt='Stran za prilagajanje gradnika z izbranim potrditvenim poljem Onemogoči obvestilno zvonec'; title='Onemogoči obvestilno zvonec' app-screenshot-end]