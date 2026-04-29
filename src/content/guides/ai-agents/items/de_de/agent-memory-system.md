Agent memory ist ein mandantenweit (tenant-scoped), **gemeinsam genutzter** Schlüssel-Wert-Pool, auf den jeder Agent in Ihrem Mandanten lesend und schreibend zugreifen kann. Er existiert, damit Agenten Kontext über mehrere Ausführungen hinweg bewahren können.

### Warum Memory existiert

Der LLM-Kontext ist pro Lauf. Ohne Memory hat ein Agent, der einem Benutzer eine Warnung ausspricht, beim nächsten Zusammentreffen mit demselben Benutzer keine Möglichkeit, sich an diese Warnung zu erinnern. Die Eskalationsrichtlinie der Plattform – "vor dem Bann warnen" – hängt davon ab, dass der Agent die vorherige Warnung finden kann. Memory ist das, was das ermöglicht.

### Zwei Arten von Memory

- **WARNING** - wird automatisch im Rahmen des [`warn_user`](#tool-warn-user)-Ablaufs geschrieben. Der Agent schreibt `WARNING`-Einträge nicht manuell; sie sind eine Nebenwirkung des Verwarnens eines Benutzers.
- **NOTE** - wird von [`save_memory`](#tools-overview) geschrieben. Allgemeiner Kontext, den der Agent zukünftigen Agenten mitteilen möchte.

Die Eskalationsrichtlinie sucht beim Abwägen, ob ein Bann gerechtfertigt ist, speziell nach `WARNING`-Einträgen.

### Mandantenweit, von Agenten geteilt

Alle Agenten in Ihrem Mandanten teilen sich **einen Memory-Pool**. Eine von Agent A gespeicherte Notiz ist für `search_memory`-Aufrufe von Agent B sichtbar. Das ist beabsichtigt – die Notizen eines Triage-Agenten sollen die Entscheidungen eines Moderations-Agenten informieren.

`tenantId` wird vom Executor aus dem eigenen Mandanten des Agenten gesetzt – niemals aus LLM-Argumenten – sodass Speicherlecks zwischen Mandanten durch Konstruktion ausgeschlossen sind.

### Was in einem Memory-Eintrag enthalten ist

Jeder Memory-Eintrag enthält:

- **Welcher Agent ihn geschrieben hat**, und wann.
- **Wen er betrifft** – den Benutzer, den dieses Memory beschreibt. Der Agent kann dies nicht erfinden; die Plattform füllt es automatisch aus dem auslösenden Ereignis.
- **Ein verborgenes Signal für Alt-Accounts** – die Plattform speichert (privat) auch den IP-Fingerabdruck des ursächlichen Kommentars, sodass zukünftige Memory-Suchen Notizen über andere Accounts anzeigen können, die von derselben IP gepostet haben. Der Fingerabdruck wird dem Agenten oder dem LLM nie gezeigt.
- **Die Notiz selbst** – bis zu 2000 Zeichen freier Text.
- **Tags** zur Auffindung – bis zu 10 kurze Tags.
- **Eine Art** – entweder eine Warning oder eine allgemeine Notiz.
- **Ein optionaler Kommentar-Link** – falls das Memory an einen bestimmten Kommentar gebunden ist.

### Suchverhalten

[`search_memory`](#tools-overview) gibt bis zu 25 Einträge zurück, sortiert nach Neuheit zuerst, automatisch scoped auf (den auslösenden Benutzer) ODER (andere Accounts auf der IP des Auslösers). Die Ergebnisse sind außerdem auf insgesamt 8000 Zeichen über alle zurückgegebenen Inhalte begrenzt – ältere Einträge werden entfernt, wenn das Limit erreicht ist.

Der Agent übergibt nicht `userId` oder `targetIpHash`. Beide werden vom Executor gesetzt.

### Persistenz

Memory hat **keine TTL**. Einträge bleiben bestehen, bis sie ausdrücklich entfernt werden. WARNING-Einträge über einen Benutzer werden absichtlich niemals automatisch gelöscht – die Eskalationshistorie muss dauerhaft auffindbar sein, sonst ist die Überprüfung "suchen bevor gebannt wird" der Plattform bedeutungslos.

Die drei Wege, wie Memory entfernt wird:

- Ein Moderator löscht den zugrundeliegenden Kommentar – alle an diesen Kommentar gebundenen Memory-Einträge werden mitgelöscht.
- Ein Benutzer wird gelöscht – alle Memory-Einträge über diesen Benutzer werden in derselben Transaktion entfernt.
- Ihr Mandant wird gelöscht.

Es gibt heute keine Admin-Oberfläche zum Löschen einzelner Memory-Einträge.

### Memory im Dry-Run

Dry-run-Agenten schreiben **kein** Memory. Das ist so beabsichtigt: Die hypothetischen Entscheidungen eines Dry-run-Agenten sollten den gemeinsamen Memory-Pool nicht verschmutzen. Das Lesen via `search_memory` funktioniert im Dry-Run normal – der Agent kann echte Memories von Live-Agenten sehen – er kann sie nur nicht ergänzen.

### Memory in Replays

Gleich wie beim Dry-Run: Replay-Agenten schreiben kein Memory. Replays sind nur eine Vorschau. siehe [Test Runs (Replays)](#test-runs-replays).

### Zusammenfassung der Beschränkungen

| Limit | Value |
|---|---|
| Memory content max length | 2000 chars |
| Memory tag max length | 64 chars |
| Memory tags max count | 10 |
| Memory query max length | 200 chars |
| Memory search result limit | 25 records |
| Memory search total content cap | 8000 chars |

### Siehe auch

- [Tool: save_memory](#tools-overview) zum Schreiben.
- [Tool: search_memory](#tools-overview) zum Lesen.
- [Tool: warn_user](#tool-warn-user) – das einzige Tool, das WARNING-artiges Memory schreibt.
- [Tool: ban_user](#tool-ban-user) – das Systemprompt verlangt, dass `search_memory` davor aufgerufen wird.