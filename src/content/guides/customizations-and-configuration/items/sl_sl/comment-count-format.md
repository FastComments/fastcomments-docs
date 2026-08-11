[related-parameter-start name = 'commentCountFormat'; type = 'string'; related-parameter-end]

Število komentarjev, prikazano na vrhu gradnika za komentarje, je mogoče prilagoditi.

To lahko nadomestite s katerimkoli nizom, vrednost **[count]** pa bo zamenjana s številom komentarjev, lokaliziranim za uporabnika.

[code-example-start config = {commentCountFormat: "There are [count] comments."}; linesToHighlight = [6]; title = 'Prilagajanje besedila števila komentarjev'; code-example-end]

To je mogoče prilagoditi brez kode na strani za prilagajanje gradnika:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.comment-count'; alt='Polje besedila števila komentarjev na strani za prilagajanje gradnika, kjer je [count] nadomeščen s trenutnim skupnim številom'; title='Prilagajanje besedila števila komentarjev' app-screenshot-end]