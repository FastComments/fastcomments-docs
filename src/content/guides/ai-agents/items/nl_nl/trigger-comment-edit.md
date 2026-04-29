Activeert de agent wanneer een opmerking wordt bewerkt.

### Context die de agent ontvangt

- De opmerking in zijn huidige (na-bewerking) vorm.
- De **vorige tekst van de opmerking** als een apart fenced block (`PREVIOUS_TEXT`). Dit is uniek voor de bewerk-trigger - de agent kan vóór/na vergelijken.
- Optionele thread / gebruikersgeschiedenis / paginacontext zoals geconfigureerd.

### Opmerkelijk

- De trigger wordt geactiveerd bij elke succesvolle bewerking, inclusief bewerkingen die door moderatoren namens een gebruiker zijn uitgevoerd.
- Agents hebben geen tool om opmerkingen te bewerken; agents kunnen helemaal geen opmerkingen bewerken.
- De vorige tekst van de opmerking is afgegrensd als onbetrouwbare invoer. De systeemprompt van het platform herinnert het model eraan geen instructies uit binnen fences te volgen - dit is hier van belang, omdat een kwaadaardige gebruiker een opmerking zou kunnen bewerken om een payload zoals "ignore your previous instructions" in te voegen die gericht is op elk agent dat naar bewerkgebeurtenissen kijkt.

### Veelvoorkomende toepassingen

- **Opsporen van gecamoufleerde inhoud** - een gebruiker bewerkt een eerder schone opmerking om spam toe te voegen nadat de moderator verder is gegaan.
- **Bijhouden van kleinere bewerkingen** - als je gemeenschap bewerkingen als afzonderlijke gebeurtenissen behandelt voor auditdoeleinden.

### Opmerking over kosten

Bewerk-triggers zien twee kopieën van de tekst van de opmerking (de nieuwe versie in het standaard COMMENT-blok, de oude versie in het PREVIOUS_TEXT-blok). Voor lange opmerkingen verdubbelt dit ruwweg het aantal tokens van de run ten opzichte van een `COMMENT_ADD` trigger - houd daar rekening mee bij het budgetteren.

---