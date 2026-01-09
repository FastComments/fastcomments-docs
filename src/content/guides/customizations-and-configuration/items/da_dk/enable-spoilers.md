[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Vi kan aktivere spoiler-understøttelse ved at sætte **enableSpoilers**-flaget til true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Enabling Spoilers'; code-example-end]

Dette kan også gøres uden kode. På siden til tilpasning af widgetten, se indstillingen "Aktivér spoilere".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; title='Enable Spoilers' app-screenshot-end]

Når tekst markeres, og den nu synlige `SPOILER`-knap klikkes, vil teksten blive maskeret, indtil brugeren holder musen over den. For mørk tilstand gør vi det samme, med forskellige
farver, der passer bedre til mørk tilstand.

Dette er også kompatibelt med WYSIWYG-editoren.