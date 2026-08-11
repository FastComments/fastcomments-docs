---
[related-parameter-start name = 'maxCommentCharacterLength'; type = 'number'; related-parameter-end]

Det maksimale antal tegn, der må indtastes i kommentarfeltet, kan begrænses af **maxCommentCharacterLength**-parameteren.

Standardværdien er 2000.

Elementer som billed-URL'er medtages ikke i længdebestemmelsen.

[code-example-start config = {maxCommentCharacterLength: 500}; linesToHighlight = [6]; title = 'Begræns kommentarlængde'; code-example-end]

Dette kan tilpasses uden kode på widgettilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.max-comment-size'; alt='Feltet for maksimal kommentarlængde på widgettilpasningssiden, der bruges til at begrænse, hvor mange tegn en kommentar kan indeholde'; title='Begræns kommentarlængde' app-screenshot-end]

---