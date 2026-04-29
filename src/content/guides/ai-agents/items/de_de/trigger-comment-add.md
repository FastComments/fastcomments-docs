Löst den Agenten jedes Mal aus, wenn ein neuer Kommentar auf einer vom [Geltungsbereich](#scope-url-locale) des Agenten abgedeckten Seite gepostet wird.

### Kontext, den der Agent erhält

- Der neue Kommentar vollständig - Text, Autor, Stimmen, Eltern-ID, Seiten-URL-ID.
- Optional: übergeordneter Kommentar und vorherige Antworten im selben Thread, wenn der [Thread-Kontext](#context-options) aktiviert ist.
- Optional: Vertrauensfaktor des Kommentierenden, Kontoalter, Sperrverlauf und jüngste Kommentare, wenn der [Benutzerhistorie-Kontext](#context-options) aktiviert ist.
- Optional: Seitenmetadaten, wenn der [Seitenkontext](#context-options) aktiviert ist.

### Bemerkenswert

- Der Auslöser wird **nach** der Speicherung des Kommentars ausgelöst. Der Agent kann direkt in Tool-Aufrufen darauf verweisen.
- Er wird **nicht** für Kommentare ausgelöst, die von einem anderen Agenten im selben Mandanten verfasst wurden.
- Er wird sowohl für verifizierte als auch nicht verifizierte Kommentare ausgelöst. Wenn Ihr Mandant vor der Sichtbarkeit eines Kommentars eine Moderatorfreigabe verlangt (siehe [Wie Genehmigungen funktionieren](/guide-moderation.html#moderation-approvals) im Moderationsleitfaden), wird der Auslöser beim Erstellen des Kommentars ausgelöst, nicht erst bei dessen späterer Genehmigung. Der Moderator-Bot kann angewiesen werden, Kommentare nach Prüfung für Sie freizugeben.

### Häufige Verwendungszwecke

- **Moderation** - prüft den Kommentar anhand der Community-Richtlinien, markiert Spam oder warnt Erstkommentatoren.
- **Begrüßung** - obwohl [Auslöser: Erster Kommentar eines neuen Nutzers](#trigger-new-user-first-comment) in der Regel besser für Begrüßungen geeignet ist, da er einmal pro Nutzer ausgelöst wird.
- **Thread-Zusammenfassung** - wird üblicherweise mit einer [Trigger-Verzögerung](#trigger-deferred-delay) kombiniert, damit sich der Thread beruhigt, bevor der Agent ausgeführt wird.

---