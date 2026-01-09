[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Når du sender notifikations-e-mails, eller gengiver kommentarer i brugergrænseflader som moderationssiden, er det nyttigt at kunne linke
fra kommentaren til den side, den står på.

Hvis URL ID ikke altid er et ID, så er vi nødt til at gemme URL'en et andet sted. Det er det, "url" property er til, defineret som følger.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Et almindeligt brugsscenarie er at knytte kommentartråden til en identifikator, som en artikel, og derefter linke tilbage til en bestemt side, for eksempel:

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

URL'en bliver ikke renset for almindelige marketingparametre. Som standard gemmes præcis den aktuelle side-URL sammen med kommentaren.