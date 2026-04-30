---
**Template ID:** `gaslight_detector`

Der Gaslight-Detector überwacht Kommentaränderungen, die die Historie mitten in einer Unterhaltung umschreiben — die Art, bei der ein Autor die Bedeutung eines früheren Kommentars ändert, nachdem bereits Antworten geschrieben wurden, sodass nachfolgende Antworten aus dem Zusammenhang gerissen oder falsch erscheinen. Wenn der Agent entscheidet, dass eine Bearbeitung diese Grenze überschreitet, stellt er den Originaltext wieder her und sendet dem Autor eine DM zur Erklärung.

Dies ist eine risikoreichere Vorlage, weil sie Nutzerinhalte verändert. Lassen Sie sie länger im [Trockentest](#dry-run-mode) laufen als eine schreibgeschützte Vorlage, und stellen Sie `edit_comment` bis zu dem Zeitpunkt hinter [Genehmigung](#approval-workflow), bis Sie dem Urteil des Modells für Ihren Traffic vertrauen.

### Auslöser

- **Kommentar bearbeitet** (`COMMENT_EDIT`) - der Agent vergleicht den neuen und den vorherigen Text und entscheidet, ob die Bearbeitung bereits vorhandene Antworten verzerrt.

Siehe [Auslöser: Kommentar bearbeitet](#trigger-comment-edit) für die vollständige Nutzlast, einschließlich des vorherigen Kommentartexts und der Anzahl der Antworten zum Zeitpunkt der Bearbeitung.

### Erlaubte Tools

- [`edit_comment`](#tool-edit-comment) - wird verwendet, um den Originaltext wiederherzustellen, wenn die Bearbeitung als Gaslighting beurteilt wird.
- [`warn_user`](#tool-warn-user) - spricht eine weiche Verwarnung aus, die der Nutzer bei seinem nächsten Besuch sieht.
- [`send_dm`](#tools-overview) - der Erklärungsweg; der Nutzer erhält eine Direktnachricht, die beschreibt, warum seine Bearbeitung rückgängig gemacht wurde.

Er kann nicht sperren, als Spam markieren, abstimmen oder neue Kommentare posten — die Angriffsfläche ist absichtlich eng gehalten.

### Empfohlene Ergänzungen vor dem Livegang

- **Stellen Sie `edit_comment` hinter eine [Genehmigung](#approval-workflow).** Das Zurücksetzen eines Kommentars ist für den Autor und für alle, die die bearbeitete Version gesehen haben, sichtbar, daher ist ein False Positive peinlich. Lassen Sie Genehmigungen eingeschaltet, bis Trockentests zeigen, dass der Agent konsistent ist.
- **Präzisieren Sie das Prompt mit dem, was auf Ihrer Seite als Gaslighting gilt.** Das Standardprompt ist bewusst kurz. Geben Sie dem Modell konkrete Beispiele — „Umdrehen einer Ja/Nein-Aussage“, „Löschen einer Zahl, auf die Antworten verweisen“, „Hinzufügen eines feindseligen Satzes nachdem Antworten gepostet wurden“ — und explizite Nicht-Beispiele wie Tippfehlerkorrekturen, Formatierungsbereinigungen oder das Hinzufügen von Quellen.
- **Verwenden Sie die Anzahl der Antworten aus dem Trigger-Kontext.** Bearbeitungen an Kommentaren mit null Antworten können eine Unterhaltung nicht verzerren; das Prompt sollte dem Modell sagen, diese zu überspringen.
- **Aktivieren Sie „Include commenter's trust factor, account age, ban history, and recent comments“** in den [Kontextoptionen](#context-options). Das Modell ist deutlich weniger aggressiv, wenn es ein langjähriges vertrauenswürdiges Konto sehen kann.
- **Erwägen Sie ein kurzes Gnadenfenster für Bearbeitungen im Prompt.** Viele Bearbeitungen innerhalb der ersten 30–60 Sekunden sind Tippfehlerkorrekturen; weisen Sie das Modell an, solche schnellen Bearbeitungen zu ignorieren.

### Empfohlene Trockentestdauer

Lassen Sie es mindestens zwei Wochen mit echtem Traffic im [Trockentest](#dry-run-mode) laufen, bevor Sie auf Aktiviert umschalten, und prüfen Sie in diesem Zeitraum jede markierte Bearbeitung. Verwenden Sie [Testläufe (Wiedergaben)](#test-runs-replays), um die letzten 30 Tage von Bearbeitungen gegen den Agenten abzuspielen, bevor Sie live gehen.

---