**Vorlagen‑ID:** `tos_enforcer`

Die Moderator‑Vorlage ist der empfohlene Ausgangspunkt, wenn Ihr Ziel darin besteht, den manuellen Moderationsaufwand zu reduzieren. Sie überprüft neue und markierte Kommentare und wendet Ihre Community‑Regeln an.

### Eingebauter Anfangs-Prompt

[inline-code-attrs-start title = 'Initialer Prompt der Moderator-Vorlage'; type='text' inline-code-attrs-end]
[inline-code-start]
You are a terms-of-service enforcement agent. Review each new comment against the community guidelines. Mark clear spam or policy violations. Issue warnings for first-offense borderline content. Escalate ban decisions only for repeat or severe violations. If a comment is clearly within the rules, approve it so it becomes visible (relevant for pre-moderation tenants). Stay out of political or subjective debates, focus on the rules as written.
[inline-code-end]

Sie werden fast immer möchten, dass Sie diesen Prompt mit konkreten Beispielen dessen, was Ihre Seite erlaubt und was nicht, anreichern. Die eigene Eskalationsrichtlinie der Plattform (warnen vor Sperre, Erinnerung durchsuchen vor Sperre) ist bereits in den Systemprompt integriert, den der Agent erhält, sodass Sie diese nicht wiederholen müssen.

### Auslöser

- **New comment posted** (`COMMENT_ADD`) - der Agent prüft jeden neuen Kommentar.
- **Comment crosses a flag threshold** (`COMMENT_FLAG_THRESHOLD`, default threshold: 3) - der Agent bewertet einen Kommentar neu, den andere Benutzer markiert haben.

### Erlaubte Tools

- [`mark_comment_approved`](#tools-overview) - nützlich für Mandanten mit Vorab-Moderation, bei denen der Agent saubere Kommentare freigibt und den Rest verbirgt.
- [`mark_comment_spam`](#tools-overview)
- [`warn_user`](#tool-warn-user)
- [`ban_user`](#tool-ban-user)

Er kann keine Kommentare posten, nicht abstimmen, nicht anpinnen, nicht sperren, keine Abzeichen vergeben oder E-Mails senden – der Prompt ist absichtlich eng gefasst.

### Empfohlene Ergänzungen vor dem Livegang

- **Legen Sie [Community Guidelines](#community-guidelines) fest.** Ein paar Sätze einer schriftlich festgehaltenen Richtlinie genügen; der Agent wendet sie bei jedem Durchlauf an.
- **Platzieren Sie `ban_user` hinter einer [Genehmigung](#approval-workflow).** Dies ist in der EU‑Region standardmäßig aktiviert (siehe [EU-DSA Artikel 17‑Konformität](#eu-dsa-compliance)) und wird überall empfohlen.
- **Erwägen Sie ebenfalls, `mark_comment_spam` hinter eine Genehmigung zu stellen,** wenn Sie wenig Volumen, aber Inhalte mit hoher Tragweite haben.
- **Platzieren Sie `mark_comment_approved` hinter einer Genehmigung,** wenn Sie Vorab‑Moderation betreiben. Das Freigeben eines schlechten Kommentars setzt ihn vor Leser; sperren Sie diese Aktion, bis der Agent durch Dry‑Run Vertrauen aufgebaut hat.
- **Aktivieren Sie "Vertrauensfaktor des Kommentierenden, Kontenalter, Sperrhistorie und letzte Kommentare einbeziehen"** in den [Kontextoptionen](#context-options). Das Modell wird viel weniger aggressiv warnen, wenn es sehen kann, dass jemand ein langjähriger, gutgläubiger Nutzer ist.

### Empfohlener Dry‑Run‑Zeitraum

Führen Sie diese Vorlage mindestens eine Woche lang im [Dry‑Run](#dry-run-mode) gegen Ihren realen Traffic aus, bevor Sie sie auf 'Aktiv' umstellen. Verwenden Sie [Testläufe (Replays)](#test-runs-replays), um auch eine Vorschau gegen die vorherigen 30 Tage zu erhalten.

---