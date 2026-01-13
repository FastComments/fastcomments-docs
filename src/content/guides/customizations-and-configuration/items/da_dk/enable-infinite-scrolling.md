[related-parameter-start name = 'enableInfiniteScrolling'; type = 'boolean'; related-parameter-end]

Som standard ændrer FastComments-widgetten sin højde for at passe til alle synlige kommentarer. Paginering opnås via en "Vis næste"
knap i slutningen af den aktuelle side, da vi har fundet, at denne interaktion føles mest behagelig for de fleste brugere.

Der er dog nogle tilfælde, hvor uendelig rulning foretrækkes. For eksempel bruger vi denne funktion i vores Stream Chat-produkt.

Vi kan skjule "Vis næste"-knapperne og skifte til uendelig rulning ved at sætte **enableInfiniteScrolling**-flaget til true:

[code-example-start config = {enableInfiniteScrolling: true}; linesToHighlight = [6]; title = 'Enabling Infinite Scrolling'; code-example-end]

Dette kræver også, at der tilføjes brugerdefineret CSS. Tilføj brugerdefineret CSS for `.comments`-vælgeren for at aktivere rulning, for eksempel:

[inline-code-attrs-start title = 'Aktiver uendelig rulning'; type = 'css'; inline-code-attrs-end]
[inline-code-start]
.comments {
    max-height: 500px;
    overflow-y: auto;
}
[inline-code-end]

Et fuldt fungerende eksempel ville være:

[code-example-start config = {enableInfiniteScrolling: true, customCSS: '.comments { max-height: 500px;  overflow-y: auto; }'}; linesToHighlight = [6, 7]; title = 'Enabling Infinite Scrolling'; code-example-end]

I eksemplet ovenfor bruger vi `customCSS`-egenskaben; det anbefales dog at bruge Widget Configuration UI i stedet af hensyn til ydeevnen. [Se dokumentationen om Custom CSS.](/guide-customizations-and-configuration.html#custom-css)