[related-parameter-start name = 'disableLiveCommenting'; type = 'boolean'; related-parameter-end]

Prema zadanim postavkama, FastComments će imati omogućeno komentiranje u stvarnom vremenu.

Ovo znači da svaki preglednik niti komentara vidi isti sadržaj.

Na primjer, ako se doda komentar, taj komentar bi se trebao prikazati. Ako se komentar uredi ili ukloni,
tada će se ti komentari urediti ili ukloniti za sve preglednike niti. Isto vrijedi za glasove i sve radnje moderacije.

Međutim, možemo to onemogućiti:

[code-example-start config = {disableLiveCommenting: true}; linesToHighlight = [6]; title = 'Onemogući komentiranje uživo'; code-example-end]

Ovo se također može učiniti bez koda. Na stranici prilagodbe widgeta, pogledajte odjeljak "Onemogući komentiranje uživo".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.disable-live-commenting']; selector = '.disable-live-commenting'; alt='Odjeljak Onemogući komentiranje uživo na stranici prilagodbe widgeta, isključivanje ažuriranja niti u stvarnom vremenu'; title='Onemogući komentiranje uživo' app-screenshot-end]