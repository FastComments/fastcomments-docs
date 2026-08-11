[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Når du gengiver en kommentartråd, eller efterlader en kommentar, skal FastComments vide, hvilken side, artikel eller produkt de kommentarer tilhører.

For at gøre dette bruger vi noget, vi kalder "URL ID". Det er enten en identifikator, som en streng eller et tal, eller en URL.

Som standard, hvis du ikke angiver urlId, vil den blive sidens URL. Vi tager den aktuelle sides URL og renser den for at fjerne eventuelle almindelige marketingparametre eller sporingsidentifikatorer.

I tilfælde af tredjepartsintegrationer, som WordPress, vil vores plugin normalt bruge den identifikator, der repræsenterer den aktuelle information, der vises, som URL ID, for eksempel artikel-/side-id'et.

[code-example-start config = {urlId: 'https://example.com/page'}; linesToHighlight = [6]; title = 'Definere en brugerdefineret URL ID'; code-example-end]

En ting, vi ofte vil referere til i dette dokument, er <a href="https://fastcomments.com/auth/my-account/customize-widget/new">Widget-tilpasnings‑UI</a>.

Dette UI kan bruges til at foretage mange ændringer af kommentarfunktionen uden at bruge kode.

Når du opretter en tilpasningsregel, vil vi ofte have den til at gælde for alle sider på vores site. I nogle tilfælde vil vi dog tilpasse kommentarfunktionen på en bestemt side, enten for at anvende brugerdefineret styling eller måske gøre kommentarer for den pågældende side anonyme. Du kunne også for eksempel få live‑kommentarer til at vises med det samme på nogle sider, mens de skjules under notifikationsknapper på andre.

Alt dette er muligt via URL ID‑indtastningsfeltet på denne side, som ser ud som følger:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.url-id'; alt='URL ID-feltet bruges til at afgrænse en tilpasningsregel til én side eller til et mønster såsom */blog/*'; title='URL ID-indtastning på widget‑tilpasningssiden' app-screenshot-end]

Værdien i dette felt skal matche *urlId*-parameteren, der sendes til kommentarfunktionen. Hvis du vil have din tilpasningsregel til at være *urlId*-agnostisk, så lad dette felt stå tomt eller indtast *.

Fra 2023 accepterer `URL ID`-feltet i widget‑tilpasning nu også mønstre! For eksempel kan du have `*/blog/*` for at tilføje styling specifik for din blog og `*/store/*` for at have styling specifik for din butik, alt sammen mens du bruger samme domæne.

### Faldgruber

1. Hvis din side har hash‑parametre (som example.com#page-1) - vil dette som standard blive en del af URL ID.
2. Under migrationer, for eksempel fra WordPress til Gatsby, kan du blive nødt til at migrere URL ID‑kommentarværdierne efter den første migration. I så fald kan du kontakte os.

---