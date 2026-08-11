[related-parameter-start name = 'useSingleLineCommentInput'; type = 'boolean'; related-parameter-end]

Privzeto FastComments uporabniku omogoča vnos komentarja v poljubnem številu vrstic, do privzete omejitve znakov.

Vendar je včasih zaželeno omejiti uporabnika na vnos le ene vrstice besedila. Nekateri primeri uporabe vključujejo spletno dražbo ali klepet v živo, za kar se lahko uporabi FastComments.

Zastavico **useSingleLineCommentInput** omogočimo na naslednji način:

[code-example-start config = {useSingleLineCommentInput: true}; linesToHighlight = [6]; title = 'Enable Single-Line Comment Input'; code-example-end]

To je mogoče tudi brez kode. Na strani za prilagajanje gradnika poiščite odsek "Enable Single-Line Comment Input".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.single-line-comment-input'; selector = '.single-line-comment-input'; alt='Potrditveno polje za vnos enovrstičnega komentarja je vklopljeno na strani za prilagajanje gradnika, kar omeji vnos na eno vrstico'; title='Omogoči vnos enovrstičnega komentarja' app-screenshot-end]

Upoštevajte, da so komentarji na vsaki strani za vsako smer sortiranja vnaprej izračunani, zato imajo vse smeri sortiranja enako zmogljivost.