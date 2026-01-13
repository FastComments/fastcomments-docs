[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Når en kommentartråd vises, eller når der afgives en kommentar, har FastComments brug for at vide, hvilken side, artikel eller produkt disse kommentarer tilhører.

Til dette bruger vi noget, vi kalder "URL ID". Det er enten en identifikator, som en streng eller et tal, eller en URL.

Som standard, hvis du ikke angiver urlId, vil det blive sidens URL. Vi tager den aktuelle side-URL og renser den for at fjerne almindelige marketingparametre eller tracking-identifikatorer.

I tilfælde af tredjepartsintegrationer, som WordPress, vil vores plugin normalt bruge den identifikator, der repræsenterer den aktuelle viste information som URL ID, for eksempel artikel-/side-id.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Defining a Custom URL ID'; code-example-end]

Én ting, som vi ofte henviser til i dette dokument, er <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget-tilpasningsbrugerflade</a>.

Denne brugerflade kan bruges til at foretage mange ændringer af kommentar-widgeten uden at bruge kode.

Når du opretter en tilpasningsregel, vil vi ofte ønske, at den gælder for alle sider på vores site. Men i nogle tilfælde ønsker vi at tilpasse kommentar-widgeten på en bestemt side, enten for at anvende brugerdefineret styling, eller måske gøre kommentarer på den pågældende side anonyme. Du kan for eksempel også få live-kommentarer til at vises med det samme på nogle sider, mens de er skjult under notifikationsknapper på andre.

Alt dette er muligt via URL ID-indtastningsfeltet på denne side, som ser ud som følger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; title='URL ID Input in The Widget Customization Page' app-screenshot-end]

Værdien i dette felt bør matche *urlId*-parameteren, der gives til kommentar-widgeten. Hvis du vil have, at din tilpasningsregel skal være *urlId* agnostisk, lad dette felt stå tomt eller indtast *.

Som af 2023 tager `URL ID`-feltet i widget-tilpasningen nu også mønstre! For eksempel kan du have `*/blog/*` for at tilføje styling specifikt til din blog og `*/store/*` for styling specifikt til din butik, alt sammen på samme domæne.

### Faldgruber

1. Hvis din side har hash-parametre (som example.com#page-1) - vil dette som standard blive en del af URL ID'en.  
2. Under migrationer, for eksempel fra WordPress til Gatsby, kan det være nødvendigt at migrere kommentarværdierne for URL ID efter den indledende migration. Kontakt os i så fald.