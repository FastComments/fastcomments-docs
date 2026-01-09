[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Po zadanim postavkama FastComments dopušta korisnicima da učitavaju slike. Kad korisnik klikne tu sliku, FastComments će, po zadanim postavkama,
otvoriti novu karticu kako bi prikazao sliku u punoj veličini. Postavljanje ove zastavice na true onemogućuje ovo ponašanje:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ako ne planirate sami uhvatiti klik na sliku (vidi [onImageClicked](#callbacks)), preporučujemo da se ovo kombinira s nekim stilovima
kako bi se uklonio dojam da se slika može kliknuti.

---