---
Feltet **Fællesskabsretningslinjer** på redigeringsformularen er en valgfri politik-tekstblok, der inkluderes i bruger-rollens kontekstbesked ved hver kørsel for denne agent. Den er indhegnet som ikke-pålidelig tekst (samme indhegning som platformen bruger til kommentarkroppe og andet brugergenereret indhold), så modellen betragter det som en politisk reference, ikke som systeminstruktioner. Det er det kanoniske sted at skrive ned "hvilken adfærd der er og ikke er tilladt på dette site", så agenten anvender det konsekvent.

### Hvordan det adskiller sig fra den indledende prompt

- **Indledende prompt** - agentens rolle og beslutningsstil. "Du er en moderator. Foretræk advarsel frem for udelukkelse."
- **Fællesskabsretningslinjer** - reglerne for dit fællesskab, formuleret som politik. "Ingen personlige angreb. Ingen promoveringslinks fra konti yngre end 24 timer. Off-topic kommentarer kan blive fjernet, hvis en tråd er ophedet."

Begge flyder ind i det samme kontekstvindue, men de kommer ind på forskellige lag - den indledende prompt er en del af systemrollen, retningslinjedokumentet er indhegnet tekst i bruger-rollekontekstbeskeden. Opdelingen gør det nemmere at redigere, når du vil opdatere den ene uden at genlæse den anden.

### Hvad er et godt retningslinjedokument

Et kort, specifikt, menneskeskrevet dokument. Lister fungerer bedre end prosa:

[inline-code-attrs-start title = 'Eksempel på fællesskabsretningslinjer'; type='text' inline-code-attrs-end]
[inline-code-start]
Tilladt:
- Substantielle uenigheder, selv stærkt formulerede.
- Links til originale kilder, også fra nye konti.
- Off-topic bemærkninger, hvis den overordnede tråd tillader dem.

Ikke tilladt:
- Personlige angreb mod specifikt navngivne brugere.
- Doxxing eller deling af privat information.
- Koordineret promoveringsaktivitet (flere kommentarer, der presser det samme eksterne link).
- Kommentarer, der kun eksisterer for at afspore diskussionen.

Grænsetilfælde:
- Stærkt sprog uden en målretning. Tilladt, hvis det ikke er rettet mod en person.
- Politiske emner uden for sidens emne. Off-topic; giv først en advarsel, fjern ikke medmindre det er vedvarende.
[inline-code-end]

Agenten anvender dette ved hver kørsel. Hvis du ændrer retningslinjerne, træder ændringen i kraft ved næste udløsning - tidligere kørsel bliver ikke vurderet bagudrettet.

### Hvad du ikke skal sætte her

- **Instruktioner om outputformatering** ("svar i HTML", "brug emoji"). De hører hjemme i [indledende prompt](#personality-prompt).
- **Lokaliseret tekst.** Retningslinjedokumentet, ligesom prompten, er **kun på engelsk** af samme grund - maskinoversættelse kan ændre agentens adfærd uden varsel. Hvis du har politikker, der varierer efter lokalitet, skriv dem alle på engelsk i dette ene dokument og strukturér dokumentet som "for tysk-sprogede sider: ..."
- **Lange citater af eksterne politikker.** Omskriv. Lang kontekst koster tokens ved hver kørsel.
- **Personlige data eller hemmeligheder.** Denne tekst sendes til LLM-leverandøren ved hver kørsel.

### Længde

Feltet er begrænset til **4000 tegn** (håndhæves både af formularen og af save-routen). Token-omkostningen ved hver kørsel er proportional med længden, så selv inden for grænsen er et par hundrede ord som regel rigeligt. Hvis dine fællesskabspolitikker strækker sig over mange sider, opsummer de dele, som agenten har brug for, i et uddrag specifikt til dette felt.

### Versionering

Der er ingen indbygget versionshistorik for retningslinjedokumentet - den senest gemte værdi er den, agenten bruger. Hvis du vil have historik, kopier dokumentet til dit eget sporingssystem før hver større redigering. [Forfin prompts](#refining-prompts)-flowet kan optage ændringer til *den indledende prompt*, men versionssætter ikke retningslinjedokumentet.

---