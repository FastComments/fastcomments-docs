Der **Kontext**-Abschnitt im Bearbeitungsformular steuert, wie viele Informationen der Agent bei jedem Lauf erhält. Mehr Kontext führt zu besseren Entscheidungen, erhöht aber die Token-Kosten pro Lauf, daher sollten Sie nur das bereitstellen, was der Agent tatsächlich benötigt.

### Was immer enthalten ist

Selbst wenn alle Kontrollkästchen deaktiviert sind, enthält die Kontextnachricht des Agents:

- Den **Auslöseereignistyp** (z. B. `COMMENT_ADD`, `COMMENT_FLAG_THRESHOLD`).
- Die **Seiten-URL und URL-ID** (wenn bekannt).
- Den **Kommentar**, der den Lauf ausgelöst hat, falls vorhanden – ID, Autor-Benutzer-ID, Anzeige name des Autors, Kommentartext, Abstimmungszahlen, Flag-Anzahl, Spam/zugelassen/überprüft-Flags, Parent-ID. Die E-Mail-Adresse des Autors wird **niemals** an den LLM-Anbieter gesendet (PII-Minimierung).
- Den **vorherigen Kommentartext** für `COMMENT_EDIT`-Trigger (damit der Agent Vorher/Nachher vergleichen kann).
- Die **Richtung der Abstimmung** für `COMMENT_VOTE_THRESHOLD`-Trigger.
- Die **auslösende Benutzer-ID** und **Badge-ID** (für Moderator-Badge-Trigger).
- Den **Badge-Katalog Ihres Mandanten** (Name, Anzeigeetikett, Beschreibung), wenn der Agent berechtigt ist, Badges zu vergeben, damit der Agent eine passende auswählen kann, ohne dass Sie die Badges im Prompt ausschreiben müssen.

Alle nicht vertrauenswürdigen Texte – Kommentar-Inhalte, Autorennamen, Seitentitel, das Richtliniendokument selbst – werden in der Kontextnachricht mit Markern wie `<<<COMMENT_TEXT>>> ... <<<END>>>` abgegrenzt. Das System-Prompt der Plattform weist das Modell an, Anweisungen innerhalb dieser Abgrenzungen niemals zu befolgen. Dies ist die Prompt-Injection-Abwehr der Plattform; Sie müssen dies nicht in Ihrem Prompt wiederholen.

### Die drei Kontrollkästchen

#### Elternkommentar und vorherige Antworten im selben Thread einbeziehen

Fügt hinzu:
- Den **Elternkommentar** – ID, Autor, Text.
- **Geschwister-Antworten** – die vorherigen Antworten auf denselben Parent im selben Thread.

Nützlich für: jeden Agenten, der auf einen Kommentar im Kontext antwortet (Begrüßungsagenten, Thread-Zusammenfasser, Moderatoren, die Antworten in Konversationen lesen).

Kosten: gering bis mittel. Begrenzt durch die Anzahl der Geschwister in einem Thread.

#### Vertrauensfaktor, Kontoalter, Sperrhistorie und aktuelle Kommentare des Kommentierenden einbeziehen

Fügt den AUTHOR_HISTORY-Block hinzu:

- **Kontoalter in Tagen** seit der Anmeldung.
- **Vertrauensfaktor (0–100)** – der FastComments-Score, der zusammenfasst, wie vertrauenswürdig der Nutzer auf dieser Seite ist. Siehe die [Spam-Erkennung](/guide-moderation.html#spam-detection)-Seite im Moderationshandbuch.
- **Anzahl vorheriger Sperren.**
- **Gesamtanzahl der Kommentare** auf dieser Seite.
- **Anzahl doppelter Inhalte** – falls der Nutzer kürzlich identischen Text gepostet hat (Anti-Spam-Signal).
- **Same-IP-Cross-Account-Signal** – Anzahl der Kommentare von derselben IP unter anderen Accounts (Signal für Mehrfachaccounts). Der IP-Hash selbst wird niemals an das LLM gesendet.
- **Aktuelle Kommentare** – bis zu 5 der jüngsten Kommentare des Nutzers, jeweils auf 300 Zeichen gekürzt, als nicht vertrauenswürdiger Text abgegrenzt.

Nützlich für: jeden Moderationsagenten. Ohne diese Informationen neigt das Modell dazu, neue Accounts und langjährige vertrauenswürdige Nutzer gleichermaßen zu sperren.

Kosten: mittel. Die aktuellen Kommentare verursachen die meisten Tokens.

#### Seitentitel, Untertitel, Beschreibung und Meta-Tags einbeziehen

Fügt den PAGE_CONTEXT-Block hinzu – Titel, Untertitel, Beschreibung und alle Meta-Tags, die FastComments für die Seite erfasst hat.

Nützlich für: Begrüßungsagenten und Thread-Zusammenfasser, bei denen das Wissen, worum es auf der Seite geht, die Qualität der Ausgabe deutlich verbessert.

Kosten: gering.

### Community-Richtlinien

Das vierte Feld, **Community guidelines**, ist ein Freitext-Policy-Block, der bei jedem Lauf in der Kontextnachricht der Benutzerrolle enthalten ist und auf die gleiche Weise als nicht vertrauenswürdiger Text abgegrenzt wird wie Kommentar-Inhalte und andere vom Nutzer bereitgestellte Inhalte. Der Agent liest ihn als Richtlinientext, aber die Plattform behandelt ihn nicht als Systemanweisung. Siehe [Community-Richtlinien](#community-guidelines) für Hinweise, was Sie dort eintragen sollten.

### Kontext selektiv hinzufügen

Diese Kontrollkästchen gelten pro Agent, nicht global. Ein gängiges Muster:

- Begrüßungsagent: Seitenkontext **an**, Thread-Kontext **aus**, Benutzerverlauf **aus**.
- Moderator: Thread-Kontext **aus**, Benutzerverlauf **an**, Seitenkontext **aus**.
- Thread-Zusammenfasser: Thread-Kontext **an**, Seitenkontext **an**, Benutzerverlauf **aus**.

Streben Sie das minimale Maß an Kontext an, das ein Agent benötigt, um die tatsächlich ausgeführten Aufrufe korrekt auszuführen – zusätzlicher Kontext kostet bei jedem Lauf Tokens, selbst wenn der Agent ihn nicht nutzt.