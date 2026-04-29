**Vorlagen-ID:** `tos_enforcer`

Die Moderator-Vorlage ist der empfohlene Ausgangspunkt, wenn Ihr Ziel darin besteht, die manuelle Moderationsarbeit zu reduzieren. Sie überprüft neue und markierte Kommentare und wendet Ihre Community-Regeln an.

Sie werden fast immer das eingebaute Prompt mit konkreten Beispielen dessen, was Ihre Seite erlaubt und nicht erlaubt, ergänzen wollen. Die Eskalationsrichtlinie der Plattform selbst (erst warnen, dann sperren; vor einer Sperre das Gedächtnis durchsuchen) ist bereits in das Systemprompt eingebettet, das der Agent erhält, sodass Sie sie nicht wiederholen müssen.

### Auslöser

- **Neuer Kommentar gepostet** (`COMMENT_ADD`) - der Agent prüft jeden neuen Kommentar.
- **Kommentar überschreitet Schwellwert für Flags** (`COMMENT_FLAG_THRESHOLD`, Standard-Schwellwert: 3) - der Agent bewertet einen von anderen Benutzern markierten Kommentar erneut.

### Zugelassene Tools

- [`mark_comment_approved`](#tools-overview) - nützlich für Pre-Moderation-Mandanten, bei denen der Agent saubere Kommentare freigibt und den Rest verbirgt.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Er kann keine Kommentare posten, abstimmen, anheften, sperren, Abzeichen vergeben oder E-Mails senden – das Prompt ist bewusst eingeschränkt.

### Empfohlene Ergänzungen vor dem Livegang

- **Legen Sie [Community-Richtlinien](#community-guidelines) fest.** Ein paar Sätze schriftlicher Richtlinien genügen; der Agent wendet sie bei jedem Lauf an.
- **Schützen Sie `ban_user` durch [Genehmigung](#approval-workflow).** Dies ist in der EU-Region standardmäßig aktiviert (siehe [EU-DSA Artikel 17 Konformität](#eu-dsa-compliance)) und wird überall empfohlen.
- **Erwägen Sie ebenfalls, `mark_comment_spam` hinter einer Genehmigung zu verbergen,** wenn Sie wenig Volumen, aber inhaltlich hohe Risiken haben.
- **Schützen Sie `mark_comment_approved` durch Genehmigung, wenn Sie Pre-Moderation betreiben.** Das Freigeben eines schlechten Kommentars macht ihn für Leser sichtbar; sperren Sie diese Aktion, bis der Agent durch Trockenläufe Vertrauen aufgebaut hat.
- **Setzen Sie ein Häkchen bei "Vertrauensfaktor des Kommentierenden, Kontoalter, Sperrverlauf und letzte Kommentare einbeziehen"** in den [Kontextoptionen](#context-options). Das Modell wird deutlich weniger aggressiv warnen, wenn es sehen kann, dass jemand ein langjähriger Nutzer in gutem Glauben ist.

### Empfohlener Trockenlaufzeitraum

Führen Sie diese Vorlage mindestens eine Woche lang im [Trockenlauf](#dry-run-mode) gegen Ihren echten Traffic aus, bevor Sie sie auf Aktiv gesetzt. Verwenden Sie [Testläufe (Wiedergaben)](#test-runs-replays), um auch eine Vorschau für die vorherigen 30 Tage zu erhalten.

---