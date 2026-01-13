---
[related-parameter-start name = 'disableImageRedirect'; type = 'boolean'; related-parameter-end]

Podrazumevano, FastComments dozvoljava korisnicima da otpremaju slike. Kada korisnik klikne tu sliku, FastComments će, podrazumevano,
otvoriti novu karticu da prikaže tu sliku u punoj veličini. Podešavanje ove zastavice na true onemogućava ovo ponašanje:

[code-example-start config = {disableImageRedirect: true}; linesToHighlight = [6]; title = 'Disable Image Redirect'; code-example-end]

Ako ne planirate sami da uhvatite klik na sliku (pogledajte [onImageClicked](#callbacks)), preporučujemo da se ovo kombinuje sa nekim stilizovanjem
kako biste uklonili utisak da je slika klikabilna.

---