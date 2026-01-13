[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Privzeto FastComments uporabniku dovoli, da vnese komentar v poljubnem številu vrstic, do privzete omejitve znakov.

Vendar je morda zaželeno omejiti uporabnika na vnos le ene vrstice besedila. Nekateri primeri uporabe vključujejo spletno dražbo ali klepet v živo, za katerega se FastComments
lahko uporablja.

Zastavico **useSingleLineCommentInput** omogočimo na naslednji način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

To je mogoče narediti tudi brez kode. Na strani za prilagoditev widgeta si oglejte razdelek "Omogoči enovrstični vnos komentarja".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; title='Enable Single-Line Comment Input' app-screenshot-end]

Upoštevajte, da so komentarji na vsaki strani za vsako smer razvrščanja vnaprej izračunani, zato imajo vse smeri razvrščanja enako zmogljivost.