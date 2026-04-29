Die **Kontext**-Sektion im Bearbeitungsformular steuert, wie viele Informationen der Agent bei jedem Lauf erhält. Mehr Kontext führt zu besseren Entscheidungen, erhöht aber die Token-Kosten pro Lauf, daher sollten Sie nur das einschließen, was der Agent tatsächlich benötigt.

### Was immer enthalten ist

Selbst wenn alle Checkboxen deaktiviert sind, enthält die Kontextnachricht des Agenten:

- Den **Trigger-Ereignistyp** (z. B. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- Die **Seiten-URL und die URL-ID** (wenn bekannt).
- Den **Kommentar**, der den Lauf ausgelöst hat, falls vorhanden - ID, Autor-User-ID, Anzeigename des Autors, Kommentartext, Abstimmungszahlen, Flag-Anzahl, Spam/zugelassen/geprüft-Flags, Parent-ID. Die E-Mail des Autors wird aus Gründen der PII-Minimierung **niemals** an den LLM-Anbieter gesendet.
- Den **vorherigen Kommentartext** für `COMMENT_EDIT`-Trigger (damit der Agent Vorher/Nachher vergleichen kann).
- Die **Richtungsinformation der Stimme** für `COMMENT_VOTE_THRESHOLD`-Trigger.
- Die **auslösende User-ID** und **badge ID** (für Moderator-Badge-Trigger).

Alle unzuverlässigen Texte - Kommentartexte, Autorennamen, Seitentitel, das Guidelines-Dokument selbst - werden in der Kontextnachricht mit Markierungen wie `<<<COMMENT_TEXT>>> ... <<<END>>>` **abgegrenzt**. Das Systemprompt der Plattform weist das Modell an, Anweisungen innerhalb dieser Markierungen niemals zu befolgen. Dies ist die Prompt-Injection-Abwehr der Plattform; Sie müssen dies nicht in Ihrem Prompt wiederholen.

### Die drei Kontrollkästchen

#### Parent-Kommentar und frühere Antworten im selben Thread einbeziehen

Fügt hinzu:
- Den **Parent-Kommentar** - ID, Autor, Text.
- **Geschwister-Antworten** - die vorherigen Antworten auf denselben Parent im selben Thread.

Nützlich für: jeden Agenten, der auf einen Kommentar kontextbezogen antwortet (Begrüßungsagenten, Thread-Zusammenfasser, Moderatoren, die Antworten in Konversationen lesen).

Kosten: klein bis mittel. Begrenzung durch die Anzahl der Geschwister in einem bestimmten Thread.

#### Vertrauensfaktor des Kommentierenden, Kontostand, Bann-Historie und kürzliche Kommentare einbeziehen

Fügt den **AUTHOR_HISTORY**-Block hinzu:

- **Kontodauer in Tagen** seit der Anmeldung.
- **Trust-Faktor (0–100)** - der FastComments-Score, der zusammenfasst, wie vertrauenswürdig der Nutzer auf dieser Seite ist. Siehe die Seite [Spam Detection](/guide-moderation.html#spam-detection) im Moderationsleitfaden.
- **Anzahl vorheriger Banns.**
- **Gesamtanzahl der Kommentare** auf dieser Seite.
- **Anzahl duplizierter Inhalte** - wenn der Nutzer kürzlich identischen Text gepostet hat (Anti-Spam-Signal).
- **Same-IP Cross-Account-Signal** - Anzahl der Kommentare von derselben IP unter anderen Konten (Alt-Account-Signal). Der IP-Hash selbst wird niemals an das LLM gesendet.
- **Kürzliche Kommentare** - bis zu 5 der zuletzt verfassten Kommentare des Nutzers, jeweils auf 300 Zeichen gekürzt, als unzuverlässiger Text abgegrenzt.

Nützlich für: jeden Moderationsagenten. Ohne diese Informationen sperrt das Modell neue Konten und langjährige vertrauenswürdige Nutzer mit derselben Haltung.

Kosten: mittel. Kürzliche Kommentare tragen am meisten zu den Tokens bei.

#### Seitentitel, Untertitel, Beschreibung und Meta-Tags einbeziehen

Fügt den **PAGE_CONTEXT**-Block hinzu - Titel, Untertitel, Beschreibung und alle Meta-Tags, die FastComments für die Seite erfasst hat.

Nützlich für: Begrüßungsagenten und Thread-Zusammenfasser, bei denen das Wissen über den Seiteninhalt die Ausgabequalität erheblich verbessert.

Kosten: gering.

### Community guidelines

Das vierte Feld, **Community guidelines**, ist ein Freitext-Policy-Block, der in der Benutzer-Rollen-Kontextnachricht bei jedem Lauf enthalten ist und auf die gleiche Weise wie Kommentartexte und andere vom Benutzer bereitgestellte Inhalte als unzuverlässiger Text abgegrenzt wird. Der Agent liest ihn als Policy-Text, aber die Plattform behandelt ihn nicht als Systemanweisung. Siehe [Community Guidelines](#community-guidelines) für Hinweise, was dort stehen sollte.

### Kontext selektiv hinzufügen

Diese Checkboxen gelten pro Agent, nicht global. Ein gängiges Muster:

- Begrüßungsagent: Seitenkontext **an**, Thread-Kontext **aus**, Nutzerhistorie **aus**.
- Moderator: Thread-Kontext **aus**, Nutzerhistorie **an**, Seitenkontext **aus**.
- Thread-Zusammenfasser: Thread-Kontext **an**, Seitenkontext **an**, Nutzerhistorie **aus**.

Streben Sie nach dem minimalen Kontext, den ein Agent benötigt, um die Aufrufe, die er tatsächlich macht, korrekt auszuführen – zusätzlicher Kontext kostet bei jedem Lauf Tokens, auch wenn der Agent ihn nicht nutzt.