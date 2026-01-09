Per impostazione predefinita, FastComments consente di inserire collegamenti a qualsiasi sito esterno.

Questo può essere limitato invece a una lista desiderata di siti, o domini. Il tentativo di pubblicare un collegamento a un sito, o dominio,
non presente nella lista definita farà sì che venga mostrato un errore all'utente.

Questa convalida riguarda solo il Comment Widget e l'API. Le importazioni non sono interessate.

Questo si fa senza codice, nella pagina di personalizzazione del widget:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.restricted-link-domains-list'; selector = '.external-link-settings'; title='Restrict External Link Domains' app-screenshot-end]

---