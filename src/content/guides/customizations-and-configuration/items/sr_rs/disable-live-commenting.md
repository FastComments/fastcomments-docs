[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments ima omogućeno komentarisanje u realnom vremenu.

Ovo znači da svaki posmatrač niti komentara vidi isti sadržaj.

Na primer, ako se doda komentar, taj komentar treba da se prikaže. Ako se komentar izmeni ili ukloni,
tada će ti komentari biti izmenjeni ili uklonjeni za sve posmatrače niti. Isto važi i za glasove i sve akcije moderacije.

Međutim, možemo ovo onemogućiti:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Onemogući komentarisanje u realnom vremenu'; code-example-end]

Ovo se takođe može uraditi bez koda. Na stranici za prilagođavanje widgeta, pogledajte odeljak „Onemogući komentarisanje u realnom vremenu“.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Odeljak „Onemogući komentarisanje u realnom vremenu“ na stranici za prilagođavanje widgeta, isključivanje ažuriranja niti u realnom vremenu'; title='Onemogući komentarisanje u realnom vremenu' app-screenshot-end]

---