[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Vi kan aktivere spoiler‑understøttelse ved at sætte **enableSpoilers**‑flaget til true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Aktivering af Spoilers'; code-example-end]

Dette kan også gøres uden kode. På widget‑tilpasningssiden, se indstillingen "Enable Spoilers" option.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Widget‑tilpasningsside med afkrydsningsfeltet Enable Spoilers markeret for at tilføje SPOILER‑knappen til editoren'; title='Aktiver Spoilers' app-screenshot-end]

Når tekst er markeret, og den nu synlige `SPOILER`‑knap klikkes, vil teksten blive maskeret, indtil brugeren holder musen over den. For mørk tilstand gør vi det samme, men med andre farver, der passer bedre til mørk tilstand.

Dette er også kompatibelt med WYSIWYG‑editoren.