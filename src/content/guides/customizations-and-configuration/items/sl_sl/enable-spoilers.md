[related-parameter-start name = 'enableSpoilers'; type = 'boolean'; related-parameter-end]

Podporo za skrivanje vsebine (spoilerje) lahko omogočimo tako, da zastavico **enableSpoilers** nastavimo na true:

[code-example-start config = {enableSpoilers: true}; linesToHighlight = [6]; title = 'Omogočanje spoilerjev'; code-example-end]

To je mogoče storiti tudi brez kode. Na strani za prilagoditve pripomočka si oglejte možnost "Enable Spoilers".

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelectors = ['.enable-spoilers']; selector = '.enable-spoilers'; alt='Stran za prilagoditve pripomočka z označeno potrditveno polje Enable Spoilers, ki doda gumb SPOILER v urejevalnik'; title='Omogoči spoilerje' app-screenshot-end]

Ko je besedilo označeno in uporabnik klikne zdaj vidni gumb `SPOILER`, bo besedilo maskirano, dokler uporabnik ne premakne miške nanj. V temnem načinu naredimo enako, vendar z drugačnimi
barvami, ki bolje ustrezajo temnemu načinu.

To je združljivo tudi z urejevalnikom WYSIWYG.