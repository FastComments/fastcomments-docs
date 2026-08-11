---
Som standard er FastComments sidestørrelse `30`. Dette inkluderer svar i tråde.

Sidestørrelsen kan tilpasses i [Widget Configuration UI](https://fastcomments.com/auth/my-account/customize-widget) i varierende størrelser fra `10` til `200`.

Bemærk, at ændring af sidestørrelsen kræver at alle kommentartråde i din konto genberegnes. Dette kan tage et par minutter.

Dette kan ikke konfigureres i klient‑side widgeten, da sider beregnes på server‑siden.

Eksempelkonfiguration vises nedenfor:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.page-size'; alt='Side størrelse vælger på widget‑tilpasningssiden, hvor en værdi fra 10 til 200 kan vælges'; title='Tilpassede sidestørrelser' app-screenshot-end]

Sidestørrelser kan tilpasses globalt, eller pr. domæne, eller pr. side, ved at oprette forskellige tilpasningsregler.

Dette vil påvirke alle klienter, integrationer og frameworks, som du måtte bruge til at vise kommentarer via vores platform.
---