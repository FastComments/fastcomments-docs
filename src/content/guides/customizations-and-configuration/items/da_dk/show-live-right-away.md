[related-parameter-start name = 'showLiveRightAway'; type = 'boolean'; related-parameter-end]

Som standard er live-kommentering aktiveret. Det betyder, at hvis kommentarer tilføjes, slettes, redigeres eller fastgøres, skal ændringerne vises for alle brugere, der ser kommentartråden samtidig.

Men som standard vil de nye kommentarer blive vist under en dynamisk knap med tekst svarende til "Vis 2 nye kommentarer".

Hvis de nye kommentarer er svar direkte på siden, vises knappen øverst i kommentartråden. Hvis de er svar på en bestemt kommentar, vises knappen under den kommentar.

Dette er for at forhindre, at sidens størrelse konstant ændrer sig for brugeren, hvilket potentielt kan skabe frustration, når man prøver at gribe fat i rullebjælken.

For nogle brugstilfælde, som live-budgivning eller onlinebegivenheder, er dette ikke den ønskede opførsel - du vil måske gerne have kommenteringswidgeten til at fungere mere som et "chat"-felt, hvor nye kommentarer "vises med det samme".

Deraf navnet på flaget, der aktiverer denne funktion: **showLiveRightAway**.

Vi kan aktivere det som følger:

[code-example-start config = {showLiveRightAway: true}; linesToHighlight = [6]; title = 'Show Live Comments Right Away'; code-example-end]

Dette kan tilpasses uden kode, på widget-tilpasningssiden:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.collapse-live-comments'; selector = '.collapse-live-comments'; title='Show Live Comments Right Away' app-screenshot-end]