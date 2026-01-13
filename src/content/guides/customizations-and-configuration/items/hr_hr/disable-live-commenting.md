[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama, FastComments ima omogućeno komentiranje uživo.

To znači da će svaki gledatelj niti komentara vidjeti isti sadržaj.

Na primjer, ako se doda komentar, taj će komentar biti prikazan. Ako se komentar uredi ili ukloni,
ti će komentari biti uređeni ili uklonjeni za sve gledatelje niti. Isto vrijedi za glasove i sve radnje moderiranja.

Međutim, možemo to onemogućiti:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ovo se može učiniti i bez koda. Na stranici za prilagodbu widgeta pogledajte odjeljak "Onemogući komentiranje uživo".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]

---