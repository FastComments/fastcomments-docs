[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ima omogućeno živo komentarisanje.

To znači da svaki gledalac niti komentara treba da vidi isti sadržaj.

Na primer, ako se komentar doda, taj komentar će se prikazati. Ako se komentar izmeni ili ukloni,
onda će ti komentari biti izmenjeni ili uklonjeni za sve gledaoce niti. Isto važi za glasove i sve moderacione akcije.

Međutim, ovo možemo onemogućiti:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Disable Live Commenting'; code-example-end]

Ovo se može uraditi i bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak "Disable Live Commenting".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; title='Disable Live Commenting' app-screenshot-end]