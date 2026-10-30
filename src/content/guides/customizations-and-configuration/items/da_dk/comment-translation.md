---
Som standard oversætter FastComments widgetens eget interface (knapper, etiketter, fejlmeddelelser) til seerens sprog, men kommentarer vises præcis som de blev skrevet.

Kommentaroversættelse tilføjer oversættelse af selve kommentarerne, så et fællesskab, der skriver på mere end ét sprog, kan læse hver kommentar på sit eget sprog.

Der er to tilstande:

- **Automatic**: kommentarer skrevet på et andet sprog end widgetens sprog vises oversat. En linje "Translated from Spanish" vises under kommentaren med et **Show original**-link, der skifter mellem oversættelsen og den uoversatte tekst.
- **Manual**: hver kommentar får et **Translate**-link. Når man klikker på det, oversættes den pågældende kommentar til seerens sprog, med den samme **Show original**-toggle bagefter. Kommentarer, der allerede er på seerens sprog, angiver dette i stedet.

Målsproget er widgetens sprog: [Locale / Language](#locale)-indstillingen på den samme tilpasningsregel, hvis den er sat, ellers besøgerens browsersprog. Der kræves intet fra besøgeren, ingen konto, udvidelse eller ekstra software, og det fungerer på samme måde i den indlejrede widget og på selvstændige kommentarsider.

Dette slås til pr. <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">tilpasningsregel</a>, så det kan gælde for hele dit site, et domæne eller en enkelt side. Det er slukket som standard.

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.comment-translation']; selector = '.comment-translation'; alt='Kommentaroversættelse dropdown på widgetens tilpasningsside med Off, Automatic og Manual muligheder'; title='Kommentaroversættelse' app-screenshot-end]

Oversættelse er en server-side indstilling. At videregive `commentTranslation` i widgetens JavaScript-konfiguration aktiverer det ikke; reglen gør.

Hver kommentar oversættes højst én gang pr. målsprog, og resultatet gemmes sammen med kommentaren, så en kommentar læst af tusind spanske brugere kun oversættes én gang. Hvis forfatteren eller en moderator redigerer kommentaren, forkastes den gemte oversættelse, og kommentaren oversættes igen ved næste visning. Oversat tekst gennemgår den samme saniterings- og linkregler som den originale kommentar. Kommentarer, der er for lange, eller hvis sproget ikke kan detekteres, vises uoversat.

Brug faktureres som LLM‑tokens til samme pris som [AI Agents](/guide-ai-agents.html#plans-and-eligibility) og vises på samme sted i dine faktureringsdetaljer.

---